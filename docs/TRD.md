# Documento de Requisitos Técnicos y Arquitectura (TRD / ARD)
## Proyecto: Capturador de Pantalla Corporativo Open Source

| Metadato | Detalle |
| :--- | :--- |
| **Producto** | Capturador de Pantalla (Screen Capture Desktop Application) |
| **Documento** | Technical Requirements Document (TRD) / Architecture Requirements Document (ARD) |
| **Versión** | 1.0.0 |
| **Estado** | Aprobado / Base Inicial |
| **Fecha** | 2026-11-09 |
| **Autor** | Julio Cesar Cisneros Palomino |
| **Referencias** | [BRD.md](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/docs/BRD.md), [PRD.md](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/docs/PRD.md), [constitution.md](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/.specify/memory/constitution.md) |

---

## 1. Alineación Constitucional y Principios de Gobierno

En estricto cumplimiento con la [Constitución del Proyecto](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/.specify/memory/constitution.md), este documento establece las bases arquitectónicas y técnicas obligatorias:

1. **Clean Architecture**: Aislamiento total de las reglas de negocio y entidades de dominio respecto a detalles del sistema operativo, frameworks gráficos y librerías externas.
2. **SOLID**: Principios de responsabilidad única, interfaces segregadas e inversión de dependencias aplicados rigurosamente en cada módulo.
3. **Desarrollo Guiado por Pruebas (TDD)**: Ciclo Red-Green-Refactor obligatorio; toda lógica de dominio y casos de uso DEBE contar con pruebas unitarias antes de su implementación de producción.
4. **DRY, KISS y YAGNI**: Prohibición de sobreingeniería y funcionalidades anticipadas; diseño conciso y modular.
5. **Seguridad y Local-First**: Cero conexiones a internet; procesamiento en memoria local; licencias de terceros permisivas (MIT/Apache 2.0); sin secretos en código fuente.

---

## 2. Evaluación Tecnológica y Decisión Arquitectónica

### 2.1 Comparativa de Frameworks de Escritorio

| Criterio | Tauri 2.0 (Rust + Web UI) | Electron (Node + Chromium) | Python (PyQt / PySide) | Go (Wails) |
| :--- | :--- | :--- | :--- | :--- |
| **Consumo RAM en reposo** | **Bajo (~25-40 MB)** | Alto (>180-250 MB) | Medio (~60-90 MB) | Bajo (~30-50 MB) |
| **Tamaño de Binario** | **~10-15 MB** | >120-180 MB | ~40-70 MB (PyInstaller) | ~15-20 MB |
| **Seguridad Corporativa** | **Alta (Rust memory-safe, IPC aislado)** | Media (Superficie Node.js amplia) | Media (Fácil de descompilar) | Alta |
| **APIs Nativas de Pantalla** | **Excelente (Rust crates nativos de SO)** | Buena (Node native addons) | Buena (Qt C++ bindings) | Regular (Librerías Cgo) |
| **Licenciamiento** | **Permisivo (MIT / Apache 2.0)** | Permisivo (MIT) | Restrictivo (GPL / LGPL) | Permisivo (MIT) |

### 2.2 Decisión Arquitectónica: Tauri 2.0 + Rust + TypeScript / Frontend Ligero

Se adopta formalmente **Tauri 2.0** como el stack arquitectónico central por las siguientes razones de ingeniería y negocio:
1. **Rendimiento y Ligereza**: Cumple con las restricciones corporativas de memoria RAM (< 50 MB en reposo) y arranque ultra rápido (< 150 ms para el overlay).
2. **Seguridad por Diseño (Security by Design)**: Rust garantiza seguridad de memoria (memory safety sin garbage collection), ideal para el tratamiento de buffers de imagen y píxeles en memoria sin riesgo de vulnerabilidades de desbordamiento de buffer.
3. **Paridad de Plataforma Nativa**:
   - **macOS**: Soporte nativo de `ScreenCaptureKit` / `CoreGraphics` y permisos del sistema (`CGRequestScreenCaptureAccess`).
   - **Windows**: Soporte nativo de Windows Graphics Capture API y GDI+.
   - **Linux**: Soporte dual para servidores gráficos X11 (mediante `x11-clipboard` / `xcap`) y Wayland (mediante Freedesktop Desktop Portal / PipeWire).
4. **Capa de Presentación Web Reactiva**: La interfaz del overlay de selección y barra de herramientas utiliza Canvas 2D / SVG acelerado por hardware para un renderizado fluido a 60 FPS sin carga excesiva.

---

## 3. Arquitectura del Sistema (Clean Architecture)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        CAPA DE PRESENTACIÓN (UI)                            │
│   - Overlay de Selección (HTML5 Canvas 2D / Transparent Window)             │
│   - Barra de Herramientas Flotante (SVG / Componentes UI reactivos)         │
│   - Menú de Bandeja del Sistema (Native System Tray)                        │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ Comandos IPC / Eventos
┌──────────────────────────────────────▼──────────────────────────────────────┐
│                    CAPA DE ADAPTADORES E INFRAESTRUCTURA                    │
│   - Tauri IPC Bridge (Controllers / Handlers)                               │
│   - ScreenCaptureDriver (macOS: CoreGraphics/SCK, Win: WGC, Linux: X11/Portal)
│   - ClipboardDriver (Rust `arboard` / OS Native Clipboard API)              │
│   - FileSystemStorageDriver (Guardado local de PNG/JPG/WebP)                 │
│   - GlobalShortcutDriver (Tauri Global Shortcut Plugin)                     │
│   - ConfigStoreDriver (Archivo JSON en directorio de usuario)               │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ Implementa Puertos (Inversión Dependencias)
┌──────────────────────────────────────▼──────────────────────────────────────┐
│                       CAPA DE APLICACIÓN (USE CASES)                        │
│   - TriggerCaptureSessionUseCase                                            │
│   - UpdateSelectionRegionUseCase                                            │
│   - AddAnnotationStrokeUseCase                                              │
│   - UndoRedoAnnotationUseCase                                               │
│   - ObfuscateSensitiveAreaUseCase (Blur / Pixelate)                         │
│   - ExportCaptureToClipboardUseCase                                         │
│   - ExportCaptureToFileUseCase                                              │
│   - ManageShortcutPreferencesUseCase                                        │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ Orquesta Reglas de Dominio
┌──────────────────────────────────────▼──────────────────────────────────────┐
│                         CAPA DE DOMINIO (CORE)                              │
│   - Entidades: `CaptureSession`, `BoundingBox`, `AnnotationItem`, `ImageBuffer`
│   - Value Objects: `Coordinates`, `Dimensions`, `ColorRGB`, `StrokeType`    │
│   - Reglas Puras: Normalización de coordenadas, Algoritmo destructivo de   │
│     difuminado (Gaussian Blur / Pixelation Kernel), Pila de deshacer/rehacer │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Diseño Detallado de Componentes y Puertos

### 4.1 Núcleo de Dominio (Domain Core)

- **`CaptureSession`**: Entidad agregada raíz que mantiene el estado actual de la captura (`Idle`, `Selecting`, `Annotating`, `Completed`, `Cancelled`), las coordenadas del monitor activo y la colección ordenada de capas de anotación.
- **`BoundingBox`**: Value Object inmutable que calcula área, ancho, alto, punto de origen $(X_0, Y_0)$ y garantiza que las coordenadas no desborden los límites físicos del monitor ($0 \le x \le W$, $0 \le y \le H$).
- **`AnnotationItem`**: Entidad abstracta para trazos vectoriales (Rectángulo, Flecha, Línea, Lápiz, Texto, Resaltador).
- **`ObfuscationKernel`**: Algoritmo puro de transformación de píxeles en memoria (downsampling + pixelation o desenfoque gaussiano) que destruye irreversiblemente los valores RGB originales para garantizar la seguridad de datos confidenciales.

### 4.2 Puertos de Abstracción (Dependency Inversion Principle)

```rust
// Contratos de interfaces desacopladas del SO y de Tauri:

pub trait IScreenCaptureDriver: Send + Sync {
    fn capture_all_monitors(&self) -> Result<Vec<RawDisplayBuffer>, DriverError>;
    fn capture_active_monitor(&self) -> Result<RawDisplayBuffer, DriverError>;
}

pub trait IClipboardDriver: Send + Sync {
    fn write_image(&self, buffer: &ProcessedImage) -> Result<(), DriverError>;
}

pub trait IFileSystemDriver: Send + Sync {
    fn save_image(&self, path: &Path, buffer: &ProcessedImage, format: ImageFormat) -> Result<(), DriverError>;
    fn load_config(&self) -> Result<AppConfig, DriverError>;
    fn save_config(&self, config: &AppConfig) -> Result<(), DriverError>;
}

pub trait IGlobalShortcutDriver: Send + Sync {
    fn register_shortcut(&self, accelerator: &str, callback: Box<dyn Fn() + Send>) -> Result<(), DriverError>;
    fn unregister_all(&self) -> Result<(), DriverError>;
}
```

---

## 5. Requisitos Técnicos Detallados (TR)

| ID | Requisito Técnico | Componente / Capa | Regla Asociada |
| :--- | :--- | :--- | :--- |
| **TR-001** | **Gestor de Captura Multiplataforma**: Integración de captura mediante librería nativa Rust (`xcap` o driver específico por SO con fallback) para obtener matrices de píxeles RGBA a escala nativa. | Infraestructura / Drivers | FR-002, NFR-005 |
| **TR-002** | **Aceleración y Soporte HiDPI**: Mapeo exacto entre píxeles lógicos del cursor y píxeles físicos del buffer de pantalla usando el factor de escala (`devicePixelRatio`). | Dominio / Presentación | NFR-005 |
| **TR-003** | **Overlay de Baja Latencia**: Ventana transparente sin bordes (`decorations: false`, `transparent: true`, `always_on_top: true`) pre-creada en segundo plano para despliegue en < 150 ms. | Presentación (Tauri) | NFR-001 |
| **TR-004** | **Renderizado de Anotaciones en Canvas**: Motor gráfico ligero basado en Canvas 2D en la UI web con trazado de figuras vectoriales mediante coordenadas relativas a la selección. | Presentación (UI) | FR-006 a FR-011 |
| **TR-005** | **Algoritmo de Ofuscación Destructiva**: Procesamiento de pixelado por bloques (ej. matriz 10x10 px promediada) ejecutado sobre el buffer antes de la exportación; no debe ser un efecto CSS reversible. | Dominio / Image Core | BR-003, FR-012 |
| **TR-006** | **Pipeline de Exportación PNG en Memoria**: Compresión y codificación de imagen final mediante crate Rust `image` con nivel de compresión balanceado (velocidad vs tamaño de archivo). | Aplicación / Dominio | FR-014, FR-015 |
| **TR-007** | **Escritura Directa en Portapapeles**: Escritura de imagen en portapapeles sin generar archivos temporales en disco usando librería nativa (`arboard`). | Infraestructura | BR-004, FR-014 |
| **TR-008** | **Persistencia de Configuración Segura**: Almacenamiento de preferencias del usuario en formato JSON estándar en la ruta oficial del sistema (`~/.config/capturador-pantalla/config.json` en Linux/macOS, `%APPDATA%\capturador-pantalla\config.json` en Windows). | Infraestructura | BR-005, FR-018 |
| **TR-009** | **Gestión de Permisos de Sistema**: Detección y solicitud guiada de permisos de grabación de pantalla en macOS (Screen Recording) y acceso a portales en Wayland. | Infraestructura | BR-007, NFR-007 |
| **TR-010** | **Integración CI y Puertas de Calidad**: Pipeline automatizado en GitHub Actions ejecutando `cargo test`, `cargo clippy`, `cargo audit`, linters de frontend y pruebas de integración en runners de Ubuntu, Windows y macOS. | Gobernanza / CI-CD | Constitución Sec. Governance |

---

## 6. Matriz de Trazabilidad (BRD ↔ PRD ↔ TRD)

| Requisito de Negocio (BRD) | Requisito de Producto (PRD) | Requisito Técnico / Arquitectónico (TRD) |
| :--- | :--- | :--- |
| **BR-001 (Local-First Estricto)** | NFR-004, FR-015 | TR-006, TR-008 (Cero módulos de red; persistencia local) |
| **BR-002 (Licenciamiento Permisivo)** | Sección 1 (Open Source) | Sección 2.2 (Stack Tauri/Rust bajo MIT/Apache-2.0) |
| **BR-003 (Protección de Confidencial)** | FR-012 (Herramienta Blur/Pixelate) | TR-005 (Algoritmo destructivo en memoria) |
| **BR-004 (Persistencia y Portapapeles)**| FR-014, FR-015 | TR-006, TR-007 (`arboard` y codificador PNG) |
| **BR-005 (Configurabilidad Sin Root)** | FR-001, FR-018 | TR-008 (Config JSON en user space) |
| **BR-006 (Consumo Mínimo de Recursos)**| NFR-001, NFR-002, NFR-003 | TR-003 (Rust backend, proceso en reposo < 40MB) |
| **BR-007 (Paridad de Plataforma)** | NFR-007 | TR-001, TR-009 (Drivers específicos macOS, Win, Linux) |

---

## 7. Estrategia de Pruebas (TDD)

De conformidad con el Principio I de la Constitución, se implementará la siguiente pirámide de pruebas:

1. **Pruebas Unitarias de Dominio (100% de cobertura en core)**:
   - Verificación de cálculos de `BoundingBox` (normalización de arrastre inverso: derecha-a-izquierda y abajo-a-arriba).
   - Verificación de algoritmo de ofuscación (comprobar que los píxeles originales no son reconstruibles).
   - Verificación de la pila Undo/Redo en `CaptureSession`.
2. **Pruebas Unitarias de Casos de Uso (con Test Doubles/Mocks de Puertos)**:
   - Verificación de orquestación de `ExportCaptureToClipboardUseCase` simulando fallos y éxitos del driver.
   - Verificación de guardado automático con formateo de fecha y hora.
3. **Pruebas de Integración de Adaptadores**:
   - Codificación y decodificación de buffers de imagen.
   - Lectura y escritura de configuración JSON con validación de esquemas.
4. **Verificación de Calidad Estática**:
   - Ejecución de `cargo fmt -- --check` y `cargo clippy -- -D warnings`.
   - Ejecución de `cargo audit` para verificación de vulnerabilidades en dependencias.
