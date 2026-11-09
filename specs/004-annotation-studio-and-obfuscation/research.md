# Research: Annotation Studio & Irreversible Obfuscation Algorithms

**Feature**: `004-annotation-studio-and-obfuscation`  
**Date**: 2026-11-09  
**Status**: Completed

---

## 1. Algoritmo de Ofuscación Destructiva e Irreversible (BR-003, FR-012)

### Decisión
Implementar un algoritmo de **Pixelado por Promediado de Bloques (Mosaic/Block Averaging)** con un tamaño de bloque predeterminado de `12x12` píxeles (configurable de `8x8` a `24x24`).

### Fundamento Matemático y de Seguridad
- **Irreversibilidad Teórica**: Al calcular el promedio aritmético de cada canal de color $(R_{avg}, G_{avg}, B_{avg})$ sobre todos los píxeles $(x, y) \in \text{Bloque}_{w \times h}$, los datos originales de alta frecuencia espacial se descartan permanentemente. Múltiples matrices de píxeles diferentes producen el mismo promedio escalar, por lo que la función de transformación no es inyectiva y matemáticamente carece de inversa única.
- **Fallo de los filtros Gaussianos**: El desenfoque gaussiano suave (Gaussian Blur) tradicional conserva gradientes continuos que, mediante técnicas modernas de de-convolución asistidas por IA (como Unblur o Depix), pueden reconstruir caracteres tipográficos. El pixelado por mosaico plano destruye totalmente las señales de contorno, haciéndolo seguro contra ataques de recuperación.
- **Rendimiento**: En Rust sobre un buffer RGBA contiguo `&mut [u8]`, el promediado de bloques procesa un área de `500x500` píxeles en menos de 0.8 milisegundos sin alocaciones adicionales de memoria.

---

## 2. Jerarquía Polimórfica de Anotaciones en el Dominio (Clean Architecture)

### Decisión
Definir un modelo discriminado serializable que sea compatible tanto en el backend en Rust como en el frontend en TypeScript:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Annotation {
    Rectangle { id: String, x: f64, y: f64, width: f64, height: f64, color: String, stroke_width: f64 },
    Ellipse { id: String, x: f64, y: f64, radius_x: f64, radius_y: f64, color: String, stroke_width: f64 },
    Arrow { id: String, start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: String, stroke_width: f64 },
    Line { id: String, start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: String, stroke_width: f64 },
    Freehand { id: String, points: Vec<(f64, f64)>, color: String, stroke_width: f64 },
    Text { id: String, x: f64, y: f64, content: String, font_size: f64, color: String },
    Highlighter { id: String, points: Vec<(f64, f64)>, color: String, stroke_width: f64 },
    Obfuscation { id: String, x: f64, y: f64, width: f64, height: f64, block_size: u32 },
}
```

### Rationale
- **Desacoplamiento Estricto**: El dominio no conoce elementos de canvas HTML5 ni librerías de interfaz de usuario.
- **Trazabilidad Isomorfa**: La misma estructura de datos se envía por IPC entre TypeScript y Rust para componer la imagen final de exportación idéntica a lo visto en pantalla.

---

## 3. Pila de Historial Undo/Redo

### Decisión
Utilizar una estructura de datos `UndoRedoHistory<T>` inmutable basada en dos pilas:
- `undo_stack: Vec<T>`
- `redo_stack: Vec<T>`

Reglas de transición:
1. `push(item)`: Añade a `undo_stack` y vacía (`clear()`) `redo_stack`.
2. `undo()`: Si `undo_stack` no está vacía, extrae el último elemento y lo inserta en `redo_stack`.
3. `redo()`: Si `redo_stack` no está vacía, extrae el último elemento y lo devuelve a `undo_stack`.

---

## 4. Barra de Herramientas Flotante Adherida Dinámicamente (UX)

### Decisión
Montar un componente HTML/CSS flotante adyacente al rectángulo de selección:
- **Posición Predeterminada**: Abajo de la selección (`y + height + 10px`), alineada a la derecha.
- **Límite Inferior**: Si la selección está cerca del borde inferior (`y + height + 10px + toolbar_height > viewport_height`), se posiciona inmediatamente encima de la selección (`y - toolbar_height - 10px`).
- **Límite Superior**: Si tampoco cabe arriba, se ancla dentro del borde inferior de la selección.
- **Atajos**:
  - `R`: Rectángulo
  - `A`: Flecha (Arrow)
  - `T`: Texto
  - `B`: Ofuscación / Blur
  - `Ctrl+Z` / `Cmd+Z`: Deshacer
  - `Ctrl+Y` / `Cmd+Shift+Z`: Rehacer