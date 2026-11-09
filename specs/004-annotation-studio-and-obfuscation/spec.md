# Feature Specification: 004-annotation-studio-and-obfuscation

**Feature Branch**: `004-annotation-studio-and-obfuscation`  
**Created**: 2026-11-09  
**Status**: Draft  
**Input**: User description: "Crea la especificación para el estudio de anotaciones y la herramienta de ofuscación de datos conforme a FR-006 a FR-013, BR-003, TR-004, TR-005 y la Constitución. Requisitos: Barra de herramientas flotante adherida dinámicamente a la selección; herramientas de dibujo vectorial (rectángulo, elipse, flecha, línea, lápiz libre, texto, resaltador); selector de paleta corporativa y grosor; herramienta obligatoria de ofuscación destructiva irreversible (Blur/Pixelate) para credenciales y PII sensible; e historial completo Deshacer/Rehacer (Ctrl/Cmd+Z, Ctrl+Y / Cmd+Shift+Z)."

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ofuscación Destructiva e Irreversible de Información Confidencial (Priority: P1) 🎯 MVP

Como empleado corporativo que comparte capturas de pantalla de sistemas internos, quiero seleccionar cualquier área sensible dentro de mi captura (contraseñas, tokens JWT, números de tarjetas o PII) y aplicar un filtro destructivo irreversible de pixelado o desenfoque antes de compartir la imagen, garantizando que nadie pueda recuperar los valores originales ni por ingeniería inversa de imagen.

**Why this priority**: Es el pilar de seguridad y cumplimiento normativo corporativo (BR-003, Principio VI de la Constitución). Evita incidentes de seguridad y fugas de datos confidenciales en entornos empresariales.

**Independent Test**: Cargar una imagen con texto legible ("TOKEN_SECRET_123"), aplicar el kernel de ofuscación destructiva en un rectángulo sobre dicho texto, y verificar mediante inspección de valores RGB que los píxeles originales han sido promediados irreversiblemente en bloques destruyendo los gradientes y frecuencias de alta resolución.

**Acceptance Scenarios**:
1. **Given** un área rectangular delimitada sobre la pantalla con datos confidenciales, **When** el usuario selecciona la herramienta de ofuscación ("Blur/Pixelate") y arrastra sobre la zona sensible, **Then** el sistema DEBE aplicar un kernel de ofuscación destructiva irreversible que sobreescriba permanentemente la matriz de píxeles sin conservar los valores subyacentes en el buffer exportable.
2. **Given** una ofuscación aplicada, **When** se analizan los píxeles resultantes, **Then** los datos originales NO DEBEN ser recuperables mediante ningún algoritmo de inversión de contraste, de-convolución o reconstrucción digital.

---

### User Story 2 - Estudio de Herramientas Vectoriales y Paleta Corporativa (Priority: P2)

Como usuario que necesita explicar un error o guiar a un colega, quiero disponer de una barra de herramientas flotante con formas geométricas (rectángulo, elipse, flecha, línea), lápiz libre, texto y resaltador semitransparente, con opciones de color y grosor de trazo, para resaltar información relevante con claridad profesional.

**Why this priority**: Convierte la simple captura en un instrumento de comunicación técnica efectiva y documentación ágil de incidentes.

**Independent Test**: Seleccionar la herramienta "Flecha" en color Rojo con grosor medio, dibujar un trazo del punto A al punto B y verificar que se renderice un cuerpo de línea con una punta de flecha simétrica de 45 grados. Probar seguidamente el resaltador amarillo verificando que mantenga un canal alfa de transparencia para dejar legible el texto de fondo.

**Acceptance Scenarios**:
1. **Given** una selección activa, **When** el usuario suelta el cursor tras demarcar la región, **Then** una barra de herramientas flotante DEBE desplegarse inmediatamente adherida a la base o tope de la selección, manteniéndose siempre dentro de los márgenes visibles de la pantalla.
2. **Given** la barra de herramientas, **When** el usuario selecciona una herramienta vectorial (Rectángulo, Elipse, Flecha, Línea, Lápiz, Texto, Resaltador), **Then** el cursor DEBE adaptarse y permitir dibujar de forma continua y fluida a 60 FPS sobre el lienzo.
3. **Given** la paleta de color y grosor, **When** el usuario elige un color corporativo (Rojo `#ef4444`, Verde `#10b981`, Azul `#3b82f6`, Amarillo `#f59e0b`, Blanco `#ffffff`, Negro `#000000`) y un grosor (Fino 2px, Medio 4px, Grueso 8px), **Then** los trazos subsiguientes DEBEN aplicar dichos atributos fielmente.

---

### User Story 3 - Historial Completo de Deshacer y Rehacer (Priority: P3)

Como usuario que comete un error al anotar o dibujar una figura, quiero poder deshacer (`Ctrl+Z` / `Cmd+Z`) o rehacer (`Ctrl+Y` / `Cmd+Shift+Z`) cualquier trazo u ofuscación de forma secuencial sin alterar el resto de la composición.

**Why this priority**: Proporciona tolerancia a fallos y fluidez en la experiencia de usuario, eliminando la necesidad de reiniciar toda la captura ante una errata.

**Independent Test**: Aplicar tres trazos consecutivos: un rectángulo, un texto y una ofuscación. Pulsar `Cmd+Z` y verificar que la ofuscación desaparece. Pulsar `Cmd+Z` y verificar que el texto desaparece. Pulsar `Cmd+Shift+Z` y verificar que el texto reaparece intacto.

**Acceptance Scenarios**:
1. **Given** una serie de anotaciones u ofuscaciones aplicadas, **When** el usuario presiona el atajo de teclado de Deshacer (`Ctrl+Z` / `Cmd+Z`) o pulsa el botón Deshacer en la barra flotante, **Then** el sistema DEBE revertir el último elemento del historial restaurando el estado previo de visualización.
2. **Given** una operación deshecha, **When** el usuario presiona Rehacer (`Ctrl+Y` / `Cmd+Shift+Z`) o pulsa el botón Rehacer, **Then** el sistema DEBE restituir la anotación con sus parámetros idénticos.
3. **Given** una pila de Rehacer con elementos, **When** el usuario realiza una nueva acción de dibujo, **Then** la pila de Rehacer DEBE purgarse conforme a la convención estándar de interfaces de usuario.

---

## Edge Cases

- **Barra de herramientas desbordando los límites de pantalla**: Si la selección se encuentra al borde inferior de la pantalla (`y + height + toolbar_height > screen_height`), la barra flotante DEBE posicionarse automáticamente en el margen superior (`y - toolbar_height`). Si la selección ocupa toda la pantalla, la barra DEBE renderizarse en el interior del margen inferior.
- **Ofuscación solapada sobre otra ofuscación**: La aplicación sucesiva de filtros de pixelado sobre la misma zona DEBE operar destructivamente sobre los píxeles ya degradados sin provocar desbordamientos numéricos en los canales RGBA.
- **Inserción de texto vacío**: Si el usuario abre el cuadro de texto y pulsa fuera o presiona Esc sin haber introducido caracteres legibles, el elemento de texto NO DEBE agregarse a la pila de anotaciones ni ensuciar el historial.
- **Texto con caracteres especiales o emojis**: El motor de renderizado tipográfico DEBE soportar caracteres UTF-8, símbolos técnicos y emojis corporativos estándar sin romper el formateo.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-006**: El sistema DEBE proveer una barra de herramientas flotante intuitiva adherida dinámicamente a la caja de selección del usuario.
- **FR-007**: El sistema DEBE ofrecer herramientas de dibujo de Rectángulo y Elipse con borde coloreado y fondo transparente.
- **FR-008**: El sistema DEBE ofrecer una herramienta de Flecha con punta vectorial proporcional al grosor del trazo.
- **FR-009**: El sistema DEBE ofrecer herramientas de Línea recta y Lápiz de trazo libre continuo a mano alzada.
- **FR-010**: El sistema DEBE ofrecer una herramienta de Texto tipográfico editable con ajuste de tamaño y color.
- **FR-011**: El sistema DEBE ofrecer una herramienta de Resaltador con color semitransparente (opacidad aproximada al 40%) que no oculte el contenido subyacente.
- **FR-012**: El sistema DEBE implementar de forma OBLIGATORIA una herramienta de Ofuscación Destructiva (Blur / Pixelate) que degrade permanentemente la resolución de los píxeles censurados.
- **FR-013**: El sistema DEBE soportar un historial inmutable de Deshacer (`Undo`) y Rehacer (`Redo`) para todas las acciones de anotación y ofuscación.

### Non-Functional & Security Requirements (Constitution)

- **BR-003**: Protección de datos confidenciales y PII en capturas mediante ofuscación no reversible.
- **TR-004**: Renderizado vectorial en tiempo real en la capa de presentación a 60 FPS.
- **TR-005**: Composición de imagen final de alta fidelidad pixel-perfect combinando el buffer crudo y las capas de anotación.
- **CON-001 (TDD)**: Toda la lógica de algoritmos de ofuscación, jerarquía polimórfica de anotaciones y pila Undo/Redo DEBE contar con pruebas unitarias previas en rojo antes de su codificación.
- **CON-002 (Clean Architecture)**: Los modelos de anotación y el algoritmo de ofuscación DEBEN residir en el dominio puro, completamente desacoplados de Tauri o de frameworks específicos.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La herramienta de ofuscación genera un bloqueo irreversible de información donde la varianza de color dentro de bloques de censura (de al menos 8x8 píxeles) se aplana a la media aritmética de bloque, impidiendo la lectura de caracteres tipográficos.
- **SC-002**: El renderizado vectorial en tiempo real de trazos y flechas mantiene una tasa continua de 60 FPS sin retraso perceptible de cursor.
- **SC-003**: Las operaciones de Deshacer y Rehacer se ejecutan en menos de 10 milisegundos para historiales de hasta 50 elementos.
- **SC-004**: Cobertura de pruebas unitarias superior al 95% en los núcleos de ofuscación y gestión de historial.

---

## Key Entities

- **AnnotationTool** (Enum): `Select`, `Rectangle`, `Ellipse`, `Arrow`, `Line`, `Freehand`, `Text`, `Highlighter`, `Blur`.
- **AnnotationItem** (Trait / Polimorfismo): Modela una figura individual. Propiedades: `id`, `tool`, `color`, `stroke_width`. Métodos: `render(context)`, `bounds()`.
- **ObfuscationBlock** (Value Object): Representa una celda de pixelado con coordenadas y tamaño de bloque.
- **UndoRedoHistory** (Entidad de Dominio): Administra dos pilas inmutables (`undo_stack: Vec<AnnotationItem>`, `redo_stack: Vec<AnnotationItem>`).