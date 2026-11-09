# Capturador de Pantalla Corporativo Open Source 📸

Una herramienta de escritorio multiplataforma (macOS, Windows y Linux), de código abierto, ultraligera y segura por diseño (Local-First), diseñada para capturar, anotar, ofuscar información sensible y compartir secciones de pantalla al instante sin costos de licenciamiento ni riesgos de fuga de datos en entornos empresariales.

---

## 🏛️ Gobernanza y Documentación del Proyecto

El proyecto se desarrolla bajo la metodología **Spec-Driven Development (SDD)** y está regido formalmente por su Constitución y documentos de especificación:

1. **[Constitución del Proyecto](.specify/memory/constitution.md)**: Establece los principios inviolables de desarrollo de software: TDD, SOLID, Clean Architecture, DRY, KISS, YAGNI, políticas de dependencias, seguridad de la información y gobernanza de calidad.
2. **[Documento de Requisitos de Negocio (BRD)](docs/BRD.md)**: Justificación de negocio, problemática en entornos corporativos remotos, partes interesadas, reglas de negocio (BR-001 a BR-007) y alcance del proyecto (Fase 1: Imágenes, Fase 2: Video/GIF).
3. **[Documento de Requisitos de Producto (PRD)](docs/PRD.md)**: Personas de usuario, épicas, historias de usuario, requisitos funcionales (FR-001 a FR-020), no funcionales (NFR-001 a NFR-007) y especificaciones UX/UI de interacción rápida.
4. **[Documento de Requisitos Técnicos y Arquitectura (TRD / ARD)](docs/TRD.md)**: Selección tecnológica (Tauri 2.0 + Rust + TypeScript), descomposición en capas de Clean Architecture (Dominio, Casos de Uso, Puertos, Adaptadores, Presentación), matriz de trazabilidad y estrategia TDD.
5. **[Guía de Prompts SDD (Spec Kit Roadmap)](docs/SDD_PROMPTS_ROADMAP.md)**: Catálogo ordenado de prompts para ejecutar `/speckit-specify` y `/speckit-plan` en cada una de las 8 features del roadmap.

---

## 🏗️ Arquitectura de Software (Clean Architecture)

El backend en Rust y el frontend en TypeScript están estructurados en capas estrictamente desacopladas:

```text
capturador_pantalla/
├── src-tauri/src/
│   ├── domain/            # Capa 1: Núcleo de Dominio (Pure Rust, 0 dependencias externas)
│   │   ├── coordinates.rs # Value Object DisplayCoordinates
│   │   ├── dimensions.rs  # Value Object Dimensions
│   │   ├── bounding_box.rs# Value Object BoundingBox (normalización matemática y clamping)
│   │   ├── buffer.rs      # Entidad RawDisplayBuffer y enum CaptureSessionState
│   │   └── error.rs       # DomainError
│   ├── application/       # Capa 2: Casos de Uso
│   │   └── session.rs     # Gestor del ciclo de vida de captura
│   ├── ports/             # Capa 3: Contratos de Inversión de Dependencias (Traits)
│   │   └── mod.rs         # IScreenCaptureDriver, IClipboardDriver, IFileSystemDriver, IGlobalShortcutDriver
│   ├── infrastructure/    # Capa 4: Adaptadores Concretos del Sistema Operativo
│   └── presentation/      # Capa 5: Controladores y Comandos IPC de Tauri
│       └── commands.rs    # get_app_info, normalize_bounding_box, get_session_state
├── src/                   # Capa de Presentación Web (TypeScript + Vite)
│   ├── components/        # Componentes de interfaz de usuario
│   ├── services/          # Cliente IPC tipado (tauriBridge)
│   ├── types/             # Interfaces TypeScript compartidas
│   └── main.ts            # Punto de entrada frontend
```

---

## ⌨️ Atajos de Teclado y Bandeja del Sistema (System Tray)

La aplicación reside discretamente en segundo plano y responde a atajos globales del sistema operativo:

- **Atajo Predeterminado de Captura**:
  - macOS: `CommandOrControl+Shift+X` (configurable a `PrintScreen`)
  - Windows / Linux: `PrintScreen` o `Ctrl+Shift+X`
- **Menú de la Bandeja del Sistema (Tray Icon)**:
  - **Tomar Captura**: Dispara la sesión interactiva de captura inmediatamente.
  - **Preferencias**: Acceso a configuración de atajos y almacenamiento local.
  - **Salir**: Termina la aplicación de forma limpia liberando recursos de memoria y controladores globales.
- **Detección de Monitores y Permisos**:
  - Soporte nativo multimonitor con enumeración física (`xcap`) y ajuste automático de escala DPI (`scale_factor`).
  - Verificación y guía proactiva de permisos de grabación de pantalla en macOS.

---

## 🎯 Overlay Interactivo de Selección (Canvas 2D @ 60 FPS)

Al dispararse la captura, se proyecta un overlay transparente acelerado por hardware:
- **Oscurecimiento Visual**: Atenuación semitransparente al 40% con recorte instantáneo del área seleccionada.
- **Normalización Automática**: Permite arrastrar en cualquier dirección cardinal (incluyendo arrastre invertido de derecha a izquierda o de abajo hacia arriba).
- **8 Manijas de Redimensionamiento**: Ajuste milimétrico desde esquinas (`NW`, `NE`, `SE`, `SW`) y puntos medios (`N`, `E`, `S`, `W`) con cursores contextuales.
- **Desplazamiento Traslacional**: Arrastre intuitivo desde el cuerpo interior para reposicionar la selección sin alterar dimensiones.
- **Dimensiones en Tiempo Real**: Badge flotante que indica ancho y alto exactos (`W × H px`).
- **Cancelación Inmediata**: La pulsación de la tecla `Esc` descarta la selección y restaura el estado en < 100ms.

---

## 🎨 Estudio de Anotaciones y Ofuscación Destructiva (Blur / Pixelate)

Una vez delimitada la selección, se despliega una barra de herramientas flotante inteligente:
- **Herramientas Vectoriales**:
  - Rectángulo (`R`) y Elipse con bordes nítidos.
  - Flecha indicadora (`A`) y Línea recta con punta direccional calibrada.
  - Lápiz de trazo libre continuo y Resaltador semitransparente (35% opacidad).
  - Texto tipográfico editable (`T`) con tamaño legible.
- **Herramienta Obligatoria de Ofuscación Destructiva (`B`)**:
  - Algoritmo de pixelado irreversible por promediado de bloques (`ObfuscationKernel`).
  - Aplana permanentemente las variaciones de alta frecuencia (contraseñas, tokens JWT, PII), haciendo imposible su reconstrucción mediante filtros o IA.
- **Paleta y Calibre**:
  - 6 colores corporativos (Rojo, Verde, Azul, Amarillo, Blanco, Negro).
  - 3 niveles de grosor de trazo (1x fino, 2x medio, 3x grueso).
- **Historial Completo Deshacer / Rehacer**:
  - Deshacer con `Ctrl+Z` / `Cmd+Z` o botón en barra.
  - Rehacer con `Ctrl+Y` / `Cmd+Shift+Z` o botón en barra.

---

### Pruebas Automatizadas (TDD)
```bash
# Ejecutar pruebas unitarias de frontend
npm run test:frontend

# Ejecutar pruebas unitarias de backend (headless en modo aislado)
npm run test:backend

# Ejecutar toda la suite de pruebas
npm test
```

### Formateo y Análisis Estático
```bash
# Verificación de formato
npm run format:check

# Corrección de formato
npm run format
```

### Ejecución en Modo Desarrollo
```bash
npm run tauri dev
```

---

## 🚀 Hoja de Ruta (Roadmap)

### Fase 1: MVP v1.0 (Captura de Imágenes Estáticas)
- Residencia en segundo plano con ícono en la bandeja del sistema (System Tray).
- Disparo mediante atajos de teclado globales configurables por el usuario.
- Overlay de pantalla completa fluido con selector de región rectangular interactivo (soporte multimonitor y pantallas HiDPI/Retina).
- Barra de herramientas flotante con figuras vectoriales y **herramienta de difuminado/ofuscación destructiva (Blur/Pixelate)** para enmascarar contraseñas, tokens y datos personales sensibles (PII).
- Copia directa al portapapeles (`image/png`) con confirmación sutil.
- Guardado en disco local (PNG, JPG, WebP) con nomenclatura parametrizable.
- Cero conexiones de red (100% Local-First).

### Fase 2: Evolución v2.0 (Captura de Video y Animaciones)
- Grabación de sección seleccionada de pantalla en formato video (MP4/WebM).
- Generación de GIF animado optimizado para mensajería corporativa (Slack, Teams).
- Controles de inicio, pausa, finalización y recorte rápido.

---

## 📐 Principios de Ingeniería

- **TDD (Test-Driven Development)**: Ninguna funcionalidad de producción se codifica sin una prueba unitaria previa en rojo.
- **Clean Architecture**: El núcleo de dominio no depende de Tauri, ni de la interfaz web, ni de APIs específicas de ningún sistema operativo.
- **Seguridad Corporativa**: Cumplimiento normativo estricto; sin telemetría, sin subida a servidores públicos externos y con borrado destructivo de píxeles en áreas censuradas.

---

## 📄 Licencia

Este proyecto está bajo la licencia [MIT](LICENSE) - permitiendo su uso libre tanto en proyectos personales como en entornos comerciales y corporativos.
