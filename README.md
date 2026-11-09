# Capturador de Pantalla Corporativo Open Source 📸

Una herramienta de escritorio multiplataforma (macOS, Windows y Linux), de código abierto, ultraligera y segura por diseño (Local-First), diseñada para capturar, anotar, ofuscar información sensible y compartir secciones de pantalla al instante sin costos de licenciamiento ni riesgos de fuga de datos en entornos empresariales.

---

## 🏛️ Gobernanza y Documentación del Proyecto

El proyecto se desarrolla bajo la metodología **Spec-Driven Development (SDD)** y está regido formalmente por su Constitución y documentos de especificación:

1. **[Constitución del Proyecto](.specify/memory/constitution.md)**: Establece los principios inviolables de desarrollo de software: TDD, SOLID, Clean Architecture, DRY, KISS, YAGNI, políticas de dependencias, seguridad de la información y gobernanza de calidad.
2. **[Documento de Requisitos de Negocio (BRD)](docs/BRD.md)**: Justificación de negocio, problemática en entornos corporativos remotos, partes interesadas, reglas de negocio (BR-001 a BR-007) y alcance del proyecto (Fase 1: Imágenes, Fase 2: Video/GIF).
3. **[Documento de Requisitos de Producto (PRD)](docs/PRD.md)**: Personas de usuario, épicas, historias de usuario, requisitos funcionales (FR-001 a FR-020), no funcionales (NFR-001 a NFR-007) y especificaciones UX/UI de interacción rápida.
4. **[Documento de Requisitos Técnicos y Arquitectura (TRD / ARD)](docs/TRD.md)**: Selección tecnológica (Tauri 2.0 + Rust + TypeScript), descomposición en capas de Clean Architecture (Dominio, Casos de Uso, Puertos, Adaptadores, Presentación), matriz de trazabilidad y estrategia TDD.

---

## 🚀 Hoja de Ruta (Roadmap)

### Fase 1: MVP v1.0 (Captura de Imágenes Estáticas)
- Residencia en segundo plano con ícono en la bandeja del sistema (System Tray).
- Disparo mediante atajos de teclado globales configurables por el usuario.
- Overlay de pantalla completa fluido con selector de región rectangular interactivo (soporte multimonitor y pantallas HiDPI/Retina).
- Barra de herramientas flotante:
  - Figuras (rectángulo, elipse, flecha, línea recta).
  - Herramienta de texto y lápiz de trazo libre.
  - Resaltador (highlighter).
  - **Herramienta de difuminado/ofuscación destructiva (Blur/Pixelate)** para enmascarar contraseñas, tokens y datos personales sensibles (PII).
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
