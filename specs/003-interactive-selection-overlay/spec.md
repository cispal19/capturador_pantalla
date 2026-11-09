# Feature Specification: 003-interactive-selection-overlay

**Feature Branch**: `003-interactive-selection-overlay`  
**Created**: 2026-11-09  
**Status**: Draft  
**Input**: User description: "Crea la especificación para el overlay interactivo de selección de región en pantalla conforme a FR-003, FR-004, FR-005, FR-016 y NFR-001. Requisitos: Despliegue de ventana transparente a pantalla completa (transparent: true, decorations: false, always_on_top: true), interacción de selección por arrastre de ratón con oscurecimiento sutil (40%), dimensiones en tiempo real (ancho x alto px), manipulación con 8 manijas y arrastre interior, soporte para arrastre invertido normalizado, cancelación inmediata con Esc y renderizado a 60 FPS en Canvas 2D."

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Demarcación Inicial Rectangular Fluida con Arrastre de Ratón (Priority: P1) 🎯 MVP

Como usuario en un entorno corporativo, cuando activo el atajo de captura de pantalla, quiero ver un oscurecimiento suave en mi pantalla y poder arrastrar el cursor del ratón en cualquier dirección para demarcar visualmente un área de captura rectangular precisa con dimensiones en tiempo real.

**Why this priority**: Es la interacción central y primaria del producto (similar a Lightshot). Sin la capacidad de arrastrar y seleccionar un área visual, no es posible proceder a la anotación, copia o guardado.

**Independent Test**: Puede probarse de forma aislada iniciando la sesión, haciendo clic en `(100, 100)` y arrastrando hacia `(500, 400)`. La pantalla debe mostrar el oscurecimiento circundante al 40%, el rectángulo transparente iluminado y la etiqueta de dimensiones `400 x 300 px`.

**Acceptance Scenarios**:
1. **Given** la aplicación en estado `Selecting` con la ventana de overlay desplegada a pantalla completa y fondo oscurecido al 40%, **When** el usuario presiona el botón principal del ratón (MouseDown) en una coordenada `(x1, y1)` y lo arrastra hasta `(x2, y2)`, **Then** el canvas DEBE renderizar en tiempo real a 60 FPS el rectángulo delimitador con borde visible y área interior despejada.
2. **Given** un arrastre invertido donde el usuario inicia en `(500, 400)` y arrastra hacia `(100, 100)` (derecha a izquierda / abajo hacia arriba), **When** el usuario suelta el botón del ratón (MouseUp), **Then** el sistema DEBE normalizar matemáticamente las coordenadas produciendo `origin = (100, 100)` y `dimensions = 400 x 300 px` sin distorsiones geométricas.
3. **Given** una selección activa, **When** el usuario arrastra el cursor, **Then** una etiqueta textual flotante DEBE mostrar el ancho y alto en píxeles lógicos (`W x H px`) en una posición que no obstruya la visibilidad ni se salga de los márgenes de la pantalla.

---

### User Story 2 - Ajuste Fino y Reposicionamiento con 8 Manijas de Control (Priority: P2)

Como usuario que necesita capturar una región exacta de una ventana o documento, quiero poder redimensionar la caja de selección tirando de cualquiera de sus 8 manijas interactivas (4 esquinas y 4 puntos medios) o desplazar la caja completa arrastrando su interior, para corregir el encuadre sin tener que reiniciar la captura.

**Why this priority**: Evita la frustración del usuario al errar por pocos píxeles en la demarcación inicial, ofreciendo paridad de experiencia con herramientas comerciales estándar de la industria.

**Independent Test**: Con una selección existente de `200 x 200 px`, hacer clic en la manija inferior derecha (`SE`) y arrastrarla `+50px` horizontal y verticalmente. Verificar que las dimensiones se actualicen a `250 x 250 px`. Luego hacer clic dentro del cuerpo del rectángulo y arrastrar `(dx=30, dy=30)` verificando que el origen se desplace a `(x+30, y+30)` manteniendo las dimensiones intactas.

**Acceptance Scenarios**:
1. **Given** una selección rectangular fijada, **When** el cursor se posicione sobre el perímetro o esquinas del rectángulo, **Then** el sistema DEBE renderizar 8 manijas de control táctil/visual (`NW`, `N`, `NE`, `E`, `SE`, `S`, `SW`, `W`) y cambiar el cursor CSS correspondientemente (`nwse-resize`, `ns-resize`, `nesw-resize`, `ew-resize`).
2. **Given** el usuario presiona MouseDown sobre una manija específica y arrastra, **Then** la caja delimitadora DEBE expandirse o contraerse según el vector de movimiento de dicha manija, respetando los límites físicos del monitor (`clamping`) y un tamaño mínimo de `10 x 10 px`.
3. **Given** el cursor situado dentro del área interior del rectángulo delimitador, **When** el usuario arrastra con el cursor `move`, **Then** la caja de selección completa DEBE trasladarse en pantalla manteniendo inalteradas sus dimensiones de ancho y alto.

---

### User Story 3 - Cancelación Inmediata y Restauración de Pantalla (Priority: P3)

Como usuario que activó el atajo por error o decidió no continuar con la captura, quiero presionar la tecla `Escape` (Esc) para cancelar el overlay de inmediato, devolviendo el control al sistema operativo sin efectos residuales.

**Why this priority**: Es un requisito fundamental de ergonomía y confianza del usuario para no atrapar al sistema en un estado modal no deseado.

**Independent Test**: Con el overlay desplegado en cualquier momento (antes, durante o después de seleccionar una región), presionar la tecla `Esc`. La ventana transparente debe cerrarse u ocultarse, la memoria del canvas debe liberarse y el estado de la sesión debe retornar a `Idle`.

**Acceptance Scenarios**:
1. **Given** el overlay de captura activo en pantalla, **When** el usuario pulsa la tecla `Esc`, **Then** el sistema DEBE abortar la sesión inmediatamente, ocultar la ventana de selección y restaurar el estado de la aplicación a `Idle`.
2. **Given** un arrastre en progreso interrumpido por `Esc`, **Then** el sistema NO DEBE persistir buffers temporales ni disparar eventos de guardado o copiado.

---

## Edge Cases

- **Arrastre de área nula o ínfima**: Si el usuario hace un clic simple sin arrastrar (área menor a `4 x 4 px`), el sistema NO DEBE crear una selección fantasma y DEBE mantenerse en espera de un arrastre válido o resetear la selección previa.
- **Arrastre fuera de los bordes del monitor**: Si el usuario arrastra el cursor fuera de los límites de la pantalla activa, las coordenadas DEBEN limitarse (`clamping`) estrictamente a `(0, 0)` y `(max_width, max_height)` de dicho monitor para impedir desbordamientos y coordenadas negativas.
- **Monitores HiDPI / Retina**: En pantallas con `scale_factor > 1.0` (ej. 2.0 en macOS Retina), la lógica de renderizado DEBE escalar adecuadamente el Canvas 2D (`window.devicePixelRatio`) para garantizar nitidez tipográfica y trazos nítidos de 1px sin distorsión borrosa.
- **Pérdida de foco de la ventana de overlay**: Si otra aplicación del sistema fuerza el foco o se produce un cambio de ventana, el overlay DEBE mantener `always_on_top: true` durante la sesión activa o cancelarse ordenadamente si el sistema operativo restringe la superposición.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-003**: El sistema DEBE proyectar una ventana superpuesta a pantalla completa (`fullscreen: true`, `transparent: true`, `decorations: false`, `always_on_top: true`) sobre el monitor activo.
- **FR-004**: El sistema DEBE oscurecer el fondo fuera del área seleccionada con una capa semitransparente uniforme (opacidad del 40%, `#000000` con `alpha: 0.40`).
- **FR-005**: El usuario DEBE poder seleccionar un área rectangular mediante clic y arrastre con visualización fluida del recuadro en tiempo real.
- **FR-006**: El sistema DEBE normalizar matemáticamente las coordenadas de la selección ante cualquier dirección de arrastre (inclusive de derecha a izquierda o de abajo hacia arriba), garantizando siempre `width >= 0`, `height >= 0` y coordenadas de origen `(x, y)` correspondientes a la esquina superior izquierda.
- **FR-007**: El sistema DEBE proveer 8 manijas de control (`NW`, `N`, `NE`, `E`, `SE`, `S`, `SW`, `W`) para permitir el redimensionamiento fino del rectángulo seleccionado.
- **FR-008**: El sistema DEBE permitir el desplazamiento traslacional del rectángulo de selección completo al arrastrar desde el interior de la región demarcada.
- **FR-009**: El sistema DEBE mostrar en tiempo real las dimensiones lógicas de la selección actual en formato textual `[Ancho] x [Alto] px`.
- **FR-010**: El sistema DEBE abortar inmediatamente el proceso de captura y ocultar el overlay al presionar la tecla `Escape` (`Esc`).
- **FR-016**: La interfaz del overlay DEBE renderizarse a una tasa objetivo de 60 cuadros por segundo (FPS) sin parpadeos perceptibles utilizando HTML5 Canvas 2D acelerado con `requestAnimationFrame`.

### Non-Functional & Governance Requirements (Constitution)

- **NFR-001**: La latencia entre el evento de ratón y la actualización del rectángulo en pantalla NO DEBE exceder los 16.6 milisegundos (renderizado a 60 FPS).
- **CON-001 (TDD)**: Todas las operaciones matemáticas geométricas (cálculo de manijas, normalización, traslación, restricciones de bordes) DEBEN implementarse guiadas por pruebas unitarias previas en rojo.
- **CON-002 (Clean Architecture)**: La lógica geométrica del `BoundingBox` y el cálculo de manijas DEBE residir en el dominio puro, completamente desacoplada de dependencias del DOM o del backend de Tauri.
- **CON-003 (Local-First & Privacidad)**: Los fotogramas y coordenadas procesadas en el overlay NO DEBEN enviarse a la red bajo ninguna circunstancia.

### Key Entities

- **BoundingBox**: Value Object que modela la región rectangular seleccionada. Atributos: `origin: DisplayCoordinates`, `dimensions: Dimensions`, `scale_factor: f64`. Métodos: `normalize()`, `clamp(max_w, max_h)`, `resize(handle, delta_x, delta_y)`, `translate(delta_x, delta_y)`.
- **ResizeHandle**: Enumeración que representa los 8 puntos de anclaje de redimensionamiento:
  - Esquinas: `NorthWest`, `NorthEast`, `SouthEast`, `SouthWest`.
  - Puntos medios: `North`, `South`, `East`, `West`.
- **OverlayRenderState**: Entidad que describe el estado instantáneo de la vista de captura: `background_opacity`, `active_box: Option<BoundingBox>`, `is_dragging: bool`, `active_handle: Option<ResizeHandle>`, `is_moving: bool`.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La tasa de refresco durante el arrastre interactivo se mantiene de forma consistente a 60 FPS sin caídas bruscas de frames en monitores convencionales y pantallas Retina 4K.
- **SC-002**: El 100% de las selecciones realizadas en cualquier dirección cardinal (4 cuadrantes cartesianos) resultan en un `BoundingBox` canónico normalizado con coordenadas positivas.
- **SC-003**: La tecla `Esc` cancela y destruye/oculta la capa superpuesta en menos de 100 milisegundos tras la pulsación.
- **SC-004**: Cobertura de pruebas unitarias superior al 95% en los módulos de cálculo geométrico y normalización del overlay.

---

## Assumptions

- Se asume que el monitor activo sobre el cual se despliega el overlay fue identificado previamente en el caso de uso `TriggerCaptureSessionUseCase` de la Feature 002.
- La aplicación se ejecuta en Tauri v2 con aceleración gráfica por hardware habilitada por defecto en el Webview.
- La selección de pantalla se realiza con dispositivos de puntero (ratón o trackpad estándar).