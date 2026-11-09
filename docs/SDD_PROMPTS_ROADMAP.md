# Guía de Prompts SDD (Spec-Driven Development) para el Roadmap de Features
## Proyecto: Capturador de Pantalla Corporativo Open Source

| Metadato | Detalle |
| :--- | :--- |
| **Proyecto** | Capturador de Pantalla (Screen Capture Tool) |
| **Documento** | Catálogo de Prompts SDD (/speckit-specify y /speckit-plan) |
| **Versión** | 1.0.0 |
| **Fecha** | 2026-11-09 |
| **Gobernanza** | [Constitución v1.0.0](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/.specify/memory/constitution.md) |
| **Especificaciones Base** | [BRD.md](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/docs/BRD.md) \| [PRD.md](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/docs/PRD.md) \| [TRD.md](file:///Users/jcisnerosp-hm/trabajos/personales/ideas_proyectos/capturador_pantalla/docs/TRD.md) |

---

## 📌 Metodología de Uso con Spec Kit

Para cada funcionalidad, se debe seguir el ciclo riguroso de **Spec-Driven Development (SDD)**:

1. **Paso 1 - Especificar**: Ejecutar el comando `/speckit-specify` utilizando el prompt de especificación correspondiente. Esto generará el archivo `spec.md` con los requisitos, historias de usuario y criterios de aceptación.
2. **Paso 2 - Planificar**: Ejecutar el comando `/speckit-plan` utilizando el prompt de planificación técnica. Esto generará el archivo `plan.md` con la arquitectura en capas, contratos de puertos, modelos y diseño de componentes.
3. **Paso 3 - Generar Tareas**: Ejecutar el comando `/speckit-tasks` para desglosar el plan en tareas unitarias TDD en `tasks.md`.
4. **Paso 4 - Implementar**: Ejecutar `/speckit-implement` para ejecutar el desarrollo bajo el ciclo Red-Green-Refactor.

---

# 🚀 FASE 1: MVP v1.0 (Captura y Anotación de Imágenes Estáticas)

---

## Feature 001: Scaffolding del Proyecto, Tauri 2.0 y Clean Architecture Base

### Contexto y Objetivos
Establecer los cimientos del repositorio: configuración de Tauri 2.0 con Rust en el backend y TypeScript/Vite en el frontend, estructura estricta de Clean Architecture (capas de Dominio, Aplicación, Puertos, Infraestructura y Presentación), arnés de pruebas TDD (`cargo test` y `vitest`) y pipeline de CI en GitHub Actions.

### Prompt para `/speckit-specify`
```text
/speckit-specify 001-scaffolding-clean-architecture
Crea la especificación para la inicialización y arquitectura base del proyecto "Capturador de Pantalla" conforme a la Constitución del proyecto, BRD, PRD y TRD.

Requisitos a especificar:
1. Configuración del proyecto de escritorio multiplataforma utilizando Tauri 2.0 (Rust backend) y TypeScript/Vite (frontend ligero).
2. Definición formal de la estructura de directorios en capas desacopladas bajo Clean Architecture:
   - domain: Entidades inmutables, Value Objects y tipos puros de la lógica de negocio (sin dependencias externas).
   - application: Casos de uso de la aplicación (orquestación pura).
   - ports: Interfaces/traits para drivers de pantalla, portapapeles, sistema de archivos y atajos de teclado (DIP).
   - infrastructure: Adaptadores concretos y plugins de Tauri.
   - presentation: Vistas transparentes y controladores de UI.
3. Configuración de arnés de pruebas unitarias automáticas para el ciclo TDD (cargo test en Rust y vitest en TypeScript).
4. Configuración de linters, formato de código y análisis estático (clippy, rustfmt, eslint, prettier).
5. Configuración de pipeline de CI en GitHub Actions con compilación cruzada y puertas de calidad obligatorias (cargo test, clippy, cargo audit).
6. Cero código o dependencias de telemetría o comunicación de red (garantía Local-First).
```

### Prompt para `/speckit-plan`
```text
/speckit-plan
Genera el plan arquitectónico y técnico para 001-scaffolding-clean-architecture detallando:
1. Árbol completo de directorios en `src-tauri` y en `src` (frontend) respetando la separación estricta de Clean Architecture y SOLID.
2. Definición de los primeros tipos de dominio en Rust: `Coordinates`, `Dimensions`, `BoundingBox`, `RawDisplayBuffer`, `CaptureSessionStatus`.
3. Definición de los traits de abstracción de puertos en `ports/`: `IScreenCaptureDriver`, `IClipboardDriver`, `IFileSystemDriver`, `IGlobalShortcutDriver`.
4. Configuración de `tauri.conf.json` optimizada para rendimiento y seguridad corporativa (permisos mínimos de IPC, ventana principal oculta en segundo plano, soporte para ventana transparente de overlay).
5. Configuración de suites de pruebas unitarias TDD iniciales que validen la creación de entidades y la inmutabilidad de los Value Objects.
6. Workflow de CI en `.github/workflows/ci.yml` con verificación de pruebas y análisis estático en Ubuntu, Windows y macOS.
```

---

## Feature 002: Motor de Captura Nativa y Atajos Globales de Teclado

### Contexto y Objetivos
Implementar la capacidad de registrar atajos de teclado globales en el sistema operativo (ej. `PrintScreen` o `Cmd+Shift+X` / `Ctrl+Shift+X`) que disparen la captura inmediata de la pantalla activa o de múltiples monitores, congelando la imagen en memoria y preparando la sesión de captura.

### Prompt para `/speckit-specify`
```text
/speckit-specify 002-native-capture-and-global-shortcuts
Crea la especificación para el motor de captura de pantalla nativa y gestión de atajos globales conforme a FR-001, FR-002, TR-001, TR-009 y la Constitución.

Requisitos a especificar:
1. Registro y captura de atajos globales a nivel de sistema operativo sin importar la aplicación en foco (utilizando atajos configurables como Cmd+Shift+X en macOS y Ctrl+Shift+X / PrintScreen en Windows/Linux).
2. Captura de pantalla nativa en buffer de memoria de alta fidelidad:
   - macOS: Integración mediante CoreGraphics / ScreenCaptureKit gestionando permisos del sistema operativo de grabación de pantalla.
   - Windows: Integración mediante Windows Graphics Capture / GDI+.
   - Linux: Integración dual con soporte para X11 y Wayland (vía Desktop Portal / PipeWire).
3. Soporte para múltiples monitores conectados y pantallas de alta densidad (HiDPI / Retina displays), preservando el factor de escala nativo (devicePixelRatio).
4. Integración de ícono en la bandeja del sistema (System Tray / Menú de estado) con opciones: "Tomar captura", "Preferencias" y "Salir".
5. Latencia de captura instantánea (tiempo de respuesta < 150 ms entre presión del atajo y obtención del buffer en memoria).
6. Procesamiento 100% en memoria local sin generar archivos temporales en disco.
```

### Prompt para `/speckit-plan`
```text
/speckit-plan
Genera el plan arquitectónico y técnico para 002-native-capture-and-global-shortcuts detallando:
1. Diseño del caso de uso `TriggerCaptureSessionUseCase` en la capa de aplicación, coordinando el driver de atajos y el driver de captura de pantalla.
2. Implementación de los adaptadores de infraestructura para `IScreenCaptureDriver` en Rust utilizando la librería nativa multiplataforma (ej. `xcap` o driver específico con fallbacks por SO).
3. Implementación del adaptador `GlobalShortcutDriver` mediante el plugin oficial `tauri-plugin-global-shortcut`.
4. Manejo de errores corporativo: detección de falta de permisos de pantalla (ej. macOS Screen Recording) con notificación guiada al usuario sin bloqueos silenciosos.
5. Estrategia de pruebas TDD: pruebas unitarias del caso de uso con mocks de drivers, pruebas de normalización de escala HiDPI y pruebas de captura sintética.
```

---

## Feature 003: Overlay Interactivo de Selección de Región (Selection Overlay)

### Contexto y Objetivos
Desplegar una ventana transparente a pantalla completa que congele la pantalla al activar la captura, permitiendo al usuario demarcar interactivamente mediante arrastre del ratón un área rectangular precisa (`BoundingBox`), redimensionarla, reposicionarla y ver las dimensiones en tiempo real.

### Prompt para `/speckit-specify`
```text
/speckit-specify 003-interactive-selection-overlay
Crea la especificación para el overlay interactivo de selección de región en pantalla conforme a FR-003, FR-004, FR-005, FR-016 y NFR-001.

Requisitos a especificar:
1. Despliegue de ventana transparente a pantalla completa (`transparent: true`, `decorations: false`, `always_on_top: true`) que cubra el monitor activo con un oscurecimiento sutil de fondo (40% opacidad).
2. Interacción de selección mediante arrastre del ratón (clic y arrastre para demarcar el área de recorte con visualización del rectángulo en tiempo real).
3. Visualización de indicadores de dimensiones en píxeles (ancho x alto, ej. `1280 x 720 px`) junto a la caja de selección.
4. Manipulación posterior de la caja delimitadora (Bounding Box):
   - 8 manijas interactivas (esquinas y puntos medios) para redimensionar la selección.
   - Arrastre dentro de la selección para desplazar el rectángulo completo sin alterar sus dimensiones.
5. Soporte para arrastre invertido (dibujar de derecha a izquierda o de abajo hacia arriba) con normalización matemática automática de coordenadas.
6. Cancelación inmediata del modo de captura al presionar la tecla `Esc`, restaurando la pantalla sin efectos secundarios.
7. Rendimiento de renderizado a 60 FPS utilizando Canvas 2D acelerado por hardware en el frontend.
```

### Prompt para `/speckit-plan`
```text
/speckit-plan
Genera el plan arquitectónico y técnico para 003-interactive-selection-overlay detallando:
1. Diseño del componente de presentación del Overlay en TypeScript con HTML5 Canvas 2D optimizado (`requestAnimationFrame`).
2. Entidad de dominio `BoundingBox` con métodos puros: `normalize()`, `clamp(max_width, max_height)`, `resize(handle, delta)`, `move(delta_x, delta_y)`.
3. Caso de uso `UpdateSelectionRegionUseCase` y su comunicación reactiva entre la UI web y el backend de Tauri mediante eventos IPC tipados.
4. Mapeo bidireccional entre coordenadas lógicas del cursor y coordenadas físicas del buffer de pantalla en monitores Retina / 4K.
5. Suite de pruebas unitarias TDD: pruebas exhaustivas de normalización geométrica de `BoundingBox`, restricciones de bordes de pantalla y cálculos de manijas de redimensionamiento.
```

---

## Feature 004: Estudio de Anotaciones y Herramienta de Ofuscación/Blur

### Contexto y Objetivos
Proporcionar una barra de herramientas flotante intuitiva adherida a la selección con herramientas de dibujo vectorial (rectángulos, elipses, flechas, líneas, lápiz, texto, resaltador) y la **herramienta obligatoria de ofuscación destructiva (Blur/Pixelate)** para censurar datos confidenciales (passwords, tokens, PII) antes de exportar la captura.

### Prompt para `/speckit-specify`
```text
/speckit-specify 004-annotation-studio-and-obfuscation
Crea la especificación para el estudio de anotaciones y la herramienta de ofuscación de datos conforme a FR-006 a FR-013, BR-003, TR-004, TR-005 y la Constitución.

Requisitos a especificar:
1. Barra de herramientas flotante adherida a la selección con reposicionamiento dinámico (evitando salirse de los límites visibles de la pantalla).
2. Herramientas de dibujo vectorial:
   - Rectángulo y Elipse (borde con color y fondo transparente).
   - Flecha indicadora (línea con punta vectorial precisa).
   - Línea recta y Lápiz de trazo libre a mano alzada.
   - Texto tipográfico con ajuste de posición y tamaño legible.
   - Resaltador (trazos semitransparentes en amarillo/verde).
3. Selector de color (paleta corporativa rápida: Rojo, Verde, Azul, Amarillo, Blanco, Negro) y selector de grosor de trazo (fino, medio, grueso).
4. Herramienta obligatoria de Ofuscación / Difuminado (Blur / Pixelate):
   - Aplicación de filtro destructivo irreversible sobre la región seleccionada para censurar credenciales, tokens o datos personales sensibles.
   - Los datos originales NO DEBEN ser recuperables en los píxeles exportados bajo ninguna técnica de inversión gráfica.
5. Historial completo de Deshacer (`Ctrl+Z` / `Cmd+Z`) y Rehacer (`Ctrl+Y` / `Cmd+Shift+Z`) para todas las anotaciones y difuminados aplicados.
```

### Prompt para `/speckit-plan`
```text
/speckit-plan
Genera el plan arquitectónico y técnico para 004-annotation-studio-and-obfuscation detallando:
1. Jerarquía polimórfica de anotaciones en el dominio: trait/interfaz `AnnotationItem` con implementaciones `RectangleAnnotation`, `ArrowAnnotation`, `TextAnnotation`, `FreehandAnnotation`, `HighlighterAnnotation`, `ObfuscationAnnotation`.
2. Diseño del algoritmo de ofuscación destructiva en el core de Rust (`ObfuscationKernel`): técnica de pixelado por promediado de bloques o desenfoque gaussiano convolucional aplicado sobre el buffer RGBA crudo.
3. Caso de uso `AddAnnotationStrokeUseCase` y orquestación de la pila `UndoRedoStack` inmutable en el dominio.
4. Renderizado vectorial inmediato en el Canvas de presentación para previsualización interactiva a 60 FPS y composición final exacta en el backend.
5. Pruebas unitarias TDD: verificación matemática de la irreversibilidad del kernel de ofuscación, pruebas de la pila Undo/Redo y pruebas de serialización de anotaciones.
```

---

## Feature 005: Salida, Portapapeles y Exportación Local de Capturas

### Contexto y Objetivos
Completar el flujo principal de trabajo: permitir al usuario presionar `Enter` o el botón "Copiar" para recortar la imagen compuesta (con anotaciones y ofuscaciones aplicadas), transferirla de forma inmediata al portapapeles del sistema operativo (`image/png`) y cerrar el overlay, así como permitir el guardado rápido en disco local (PNG, JPG, WebP).

### Prompt para `/speckit-specify`
```text
/speckit-specify 005-output-clipboard-and-local-storage
Crea la especificación para el pipeline de exportación a portapapeles y almacenamiento local conforme a FR-014, FR-015, BR-001, BR-004, TR-006, TR-007 y la Constitución.

Requisitos a especificar:
1. Acción de Copiado Rápido al Portapapeles:
   - Disparador mediante tecla `Enter` o botón "Copiar" en la barra de herramientas.
   - Composición en memoria del recorte de pantalla base + capas de anotación vectoriales + filtros de ofuscación aplicados.
   - Escritura directa e inmediata en el portapapeles del sistema operativo en formato estándar MIME `image/png` compatible con Slack, Teams, WhatsApp, Jira y clientes de correo.
   - Cierre instantáneo del overlay tras la escritura exitosa.
2. Notificación sutil y no invasiva de confirmación (ej. notificación nativa del sistema: "Captura copiada al portapapeles").
3. Acción de Guardado en Disco Local:
   - Disparador mediante atajo `Ctrl+S` / `Cmd+S` o botón "Guardar".
   - Diálogo nativo de guardado de archivo o guardado rápido en directorio predeterminado con patrón de nomenclatura parametrizable (`Captura_YYYY-MM-DD_HH-mm-ss.png`).
   - Soporte para formatos PNG, JPG y WebP.
4. Garantía estricta de privacidad: Cero persistencia en la nube, cero solicitudes de red salientes.
```

### Prompt para `/speckit-plan`
```text
/speckit-plan
Genera el plan arquitectónico y técnico para 005-output-clipboard-and-local-storage detallando:
1. Implementación del caso de uso `ExportCaptureToClipboardUseCase` y `ExportCaptureToFileUseCase` en la capa de aplicación.
2. Implementación del adaptador `ClipboardDriver` implementando `IClipboardDriver` utilizando el crate nativo `arboard` para escritura multiplataforma sin archivos temporales.
3. Implementación del adaptador `FileSystemStorageDriver` implementando `IFileSystemDriver` con codificación de imágenes optimizada mediante el crate `image`.
4. Manejo de errores y resiliencia: captura de excepciones de bloqueo del portapapeles o disco sin generar crashes en el proceso principal.
5. Estrategia de pruebas TDD: pruebas unitarias de composición de imagen, codificación PNG/JPG/WebP en memoria con buffers sintéticos y pruebas de nomenclatura de archivos.
```

---

## Feature 006: Panel de Configuración y Preferencias de Usuario

### Contexto y Objetivos
Proporcionar una interfaz gráfica de configuración accesible desde el System Tray que permita al usuario personalizar las combinaciones de teclas de captura, la carpeta de guardado automático, el formato predeterminado y el inicio con el sistema operativo, almacenando todo en el perfil local sin requerir privilegios de administrador.

### Prompt para `/speckit-specify`
```text
/speckit-specify 006-settings-and-user-preferences
Crea la especificación para el panel de configuración y preferencias de usuario conforme a FR-017, FR-018, BR-005, TR-008 y la Constitución.

Requisitos a especificar:
1. Ventana de preferencias accesible desde el menú contextual del System Tray ("Preferencias...").
2. Configuración personalizable de atajos de teclado globales (grabador interactivo de combinaciones de teclas, validando conflictos con atajos reservados del sistema).
3. Configuración de almacenamiento local: selector de carpeta de guardado por defecto y selector de formato de imagen predeterminado (PNG, JPG, WebP).
4. Configuración de inicio automático con el sistema operativo (autostart) sin requerir permisos de administrador.
5. Persistencia de configuración en archivo JSON local en el directorio estándar del usuario (`~/.config/capturador-pantalla/config.json` en Linux/macOS, `%APPDATA%\capturador-pantalla\config.json` en Windows).
6. Aplicación dinámica de cambios en tiempo de ejecución (ej. re-registro de atajos globales inmediatos sin necesidad de reiniciar la aplicación).
```

### Prompt para `/speckit-plan`
```text
/speckit-plan
Genera el plan arquitectónico y técnico para 006-settings-and-user-preferences detallando:
1. Modelo de datos de configuración en el dominio (`AppPreferences`) con valores predeterminados seguros y validación de esquemas JSON (`serde`).
2. Caso de uso `ManageShortcutPreferencesUseCase` y `UpdateStoragePreferencesUseCase`.
3. Implementación del adaptador `JsonConfigStoreDriver` implementando `IFileSystemDriver` con validación de rutas y creación segura de directorios locales.
4. Integración del plugin `tauri-plugin-autostart` para gestión del inicio con el sistema.
5. Interfaz gráfica de preferencias construida con componentes accesibles y diseño corporativo limpio.
6. Pruebas unitarias TDD: pruebas de serialización/deserialización JSON, pruebas de validación de atajos inválidos y pruebas de migración de esquema de configuración.
```

---

# 🎥 FASE 2: Evolución v2.0 (Captura de Video y Animaciones)

---

## Feature 007: Motor de Grabación de Video en Región de Pantalla

### Contexto y Objetivos
Extender la herramienta para permitir la grabación de video de una sección seleccionada de pantalla (MP4 / WebM), incorporando controles flotantes de inicio, pausa, detención y temporizador de grabación.

### Prompt para `/speckit-specify`
```text
/speckit-specify 007-screen-recording-engine
Crea la especificación para la funcionalidad de grabación de video en región de pantalla conforme a FR-019, BR-001, BR-007 y la Constitución.

Requisitos a especificar:
1. Modo dual en el selector de región: alternar entre "Capturar Imagen" y "Grabar Video".
2. Barra de control de grabación flotante con botones: Iniciar, Pausar, Reanudar, Detener y Cancelar.
3. Temporizador visual en tiempo real que indique la duración de la grabación con límite configurable para evitar archivos sobredimensionados.
4. Marco visual sutil con borde rojo parpadeante o resaltado que delimite el área activa de grabación.
5. Codificación de video local en formatos estándar compatibles con plataformas corporativas (H.264 / MP4 y VP9 / WebM).
6. Procesamiento y compresión optimizada en segundo plano para garantizar fluidez (30/60 FPS) sin saturar la CPU de la estación de trabajo.
7. Guardado local directo en la carpeta configurada y opción de copiar el archivo de video resultante al portapapeles si el sistema operativo lo soporta.
```

### Prompt para `/speckit-plan`
```text
/speckit-plan
Genera el plan arquitectónico y técnico para 007-screen-recording-engine detallando:
1. Diseño de la arquitectura de streaming de frames: abstracción de captura de video por SO (`ScreenCaptureKit` en macOS, `Desktop Duplication` en Windows, `PipeWire` en Linux).
2. Pipeline de codificación de frames a video en Rust mediante librerías nativas ligeras o FFmpeg embebido estáticamente sin dependencias externas.
3. Caso de uso `StartScreenRecordingUseCase`, `PauseScreenRecordingUseCase`, `StopScreenRecordingUseCase` gestionando hilos asíncronos en Rust (`tokio`).
4. Máquina de estados de grabación (`Idle`, `Recording`, `Paused`, `Finalizing`) y su comunicación por eventos IPC a la UI.
5. Pruebas TDD: pruebas unitarias de la máquina de estados, pruebas de contabilidad de tiempos y pruebas de codificación de secuencias sintéticas de frames.
```

---

## Feature 008: Exportación y Optimización de GIFs Animados

### Contexto y Objetivos
Permitir a los usuarios generar animaciones GIF optimizadas y de bajo peso a partir de fragmentos breves de pantalla (< 15 segundos), ideales para compartir demostraciones de bugs o explicaciones rápidas en Slack, Teams o GitHub issues.

### Prompt para `/speckit-specify`
```text
/speckit-specify 008-gif-recording-and-optimization
Crea la especificación para la exportación y optimización de animaciones GIF conforme a FR-020, BR-001, BR-004 y la Constitución.

Requisitos a especificar:
1. Opción de exportación directa a formato GIF animado para grabaciones breves (hasta 15 segundos).
2. Algoritmo de cuantización y paleta adaptativa por frame o global para garantizar alta calidad visual sin degradación excesiva de color.
3. Optimización agresiva de tamaño de archivo (eliminación de píxeles idénticos entre frames consecutivos) para que el GIF final no supere los límites habituales de mensajería (ej. < 5-10 MB).
4. Copia directa del GIF generado al portapapeles y guardado local en disco.
5. Vista previa rápida del GIF antes de la confirmación final.
```

### Prompt para `/speckit-plan`
```text
/speckit-plan
Genera el plan arquitectónico y técnico para 008-gif-recording-and-optimization detallando:
1. Caso de uso `ExportRecordingToGifUseCase` integrando cuantización de color (algoritmo NeuQuant o Octree) en Rust.
2. Implementación del codificador GIF en la capa de adaptadores (`image` crate o `gif` crate de Rust).
3. Estrategia de limitación de tasa de muestreo (15 a 24 FPS) para balancear suavidad de movimiento y peso del archivo.
4. Pruebas unitarias TDD: verificación de generación válida de encabezados GIF89a, pruebas de compresión de frames estáticos vs móviles y pruebas de umbral de peso de archivo.
```

---

## 📋 Resumen de Ejecución Secuencial

| Orden | Código de Feature | Nombre | Fase |
| :---: | :--- | :--- | :---: |
| **1** | `001-scaffolding-clean-architecture` | Scaffolding Tauri 2.0 + Clean Architecture + CI | MVP v1.0 |
| **2** | `002-native-capture-and-global-shortcuts`| Motor de Captura Nativa y Atajos Globales | MVP v1.0 |
| **3** | `003-interactive-selection-overlay` | Overlay Interactivo de Selección de Región | MVP v1.0 |
| **4** | `004-annotation-studio-and-obfuscation` | Estudio de Anotaciones y Ofuscación (Blur) | MVP v1.0 |
| **5** | `005-output-clipboard-and-local-storage` | Salida a Portapapeles y Almacenamiento Local | MVP v1.0 |
| **6** | `006-settings-and-user-preferences` | Panel de Configuración y System Tray | MVP v1.0 |
| **7** | `007-screen-recording-engine` | Motor de Grabación de Video en Región | v2.0 |
| **8** | `008-gif-recording-and-optimization` | Exportación y Optimización de GIFs | v2.0 |
