# Feature Specification: 005-output-clipboard-and-local-storage

**Feature Branch**: `005-output-clipboard-and-local-storage`  
**Created**: 2026-11-09  
**Status**: Draft  
**Input**: User description: "Crea la especificación para el pipeline de exportación a portapapeles y almacenamiento local conforme a FR-014, FR-015, BR-001, BR-004, TR-006, TR-007 y la Constitución. Requisitos: Copiado rápido al portapapeles con tecla Enter o botón Copiar con formato MIME image/png; composición en memoria del recorte + ofuscaciones + anotaciones; cierre inmediato del overlay; notificación sutil de confirmación; guardado en disco local con Ctrl+S / Cmd+S o botón Guardar en PNG, JPG y WebP con patrón de nomenclatura Captura_YYYY-MM-DD_HH-mm-ss; y garantía estricta de privacidad Local-First sin red."

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Copiado Inmediato al Portapapeles con Tecla Enter o Botón (Priority: P1) 🎯 MVP

Como usuario en un entorno corporativo remoto, una vez delimitada el área de captura y aplicadas las anotaciones y ofuscaciones pertinentes, quiero presionar la tecla `Enter` o el botón "Copiar" para transferir la imagen final compuesta directamente al portapapeles del sistema operativo en formato `image/png` y que el overlay se cierre al instante, de modo que pueda pegarla inmediatamente en Slack, Teams, Jira o correo con `Ctrl+V` / `Cmd+V`.

**Why this priority**: Es el flujo de uso más frecuente y de mayor valor del capturador de pantalla (el 85% de las capturas se comparten pegándose directamente en chats o tickets corporativos sin guardarse en disco).

**Independent Test**: Delimitar una región rectangular de `300 x 200 px`, presionar la tecla `Enter` y verificar que el portapapeles del sistema contenga una imagen válida en formato PNG con las dimensiones exactas y que la ventana de selección se oculte en menos de 150 milisegundos.

**Acceptance Scenarios**:
1. **Given** una selección activa con anotaciones u ofuscaciones opcionales, **When** el usuario pulsa la tecla `Enter` o hace clic en el botón "Copiar", **Then** el sistema DEBE componer la imagen resultante en memoria y transferirla al portapapeles del sistema operativo como `image/png`.
2. **Given** la escritura exitosa en el portapapeles, **When** se finaliza la transferencia, **Then** el sistema DEBE cerrar/ocultar el overlay inmediatamente y retornar el estado de la sesión a `Idle`.
3. **Given** un portapapeles temporalmente ocupado o bloqueado por otra aplicación, **When** el driver intente escribir, **Then** el sistema DEBE reintentar o reportar un error amigable sin congelar el proceso principal.

---

### User Story 2 - Guardado en Disco Local con Nomenclatura Parametrizable (Priority: P2)

Como usuario que documenta evidencias técnicas o auditorías, quiero guardar la captura como archivo en el disco local de mi máquina con el atajo `Ctrl+S` / `Cmd+S` o el botón "Guardar", generando automáticamente un nombre ordenable por fecha y hora (`Captura_YYYY-MM-DD_HH-mm-ss.png`), permitiendo formatos PNG, JPG y WebP.

**Why this priority**: Esencial para archivado documental, reportes de bugs a largo plazo y manuales corporativos.

**Independent Test**: Con una selección activa, pulsar `Cmd+S`, seleccionar una ruta de destino local (o carpeta predeterminada) y verificar la creación física del archivo en disco con formato de imagen válido y dimensiones idénticas a la selección.

**Acceptance Scenarios**:
1. **Given** una selección activa, **When** el usuario presiona `Ctrl+S` / `Cmd+S` o hace clic en el botón "Guardar", **Then** el sistema DEBE persistir la imagen compuesta en la ruta indicada con el patrón de fecha y hora local.
2. **Given** la solicitud de guardado, **When** el usuario elija formato (PNG, JPEG o WebP), **Then** el codificador DEBE generar un archivo conforme a la especificación estándar del formato seleccionado con cero pérdida visual en PNG y compresión optimizada en JPEG/WebP.
3. **Given** el guardado exitoso, **When** el archivo se escribe en disco, **Then** el overlay DEBE cerrarse de forma limpia.

---

### User Story 3 - Notificación Visual Sutil de Confirmación (Priority: P3)

Como usuario que acaba de copiar o guardar una captura, quiero recibir una confirmación sutil (toast en pantalla o notificación nativa) que me garantice que la captura fue copiada o guardada con éxito, sin interrumpir mi flujo de trabajo.

**Why this priority**: Refuerza la confianza del usuario al confirmar que la imagen ya está en el portapapeles lista para pegarse.

**Independent Test**: Copiar una captura al portapapeles y verificar la aparición del mensaje de confirmación "Captura copiada al portapapeles" desapareciendo suavemente.

**Acceptance Scenarios**:
1. **Given** la copia al portapapeles o guardado en disco, **When** la operación concluye con éxito, **Then** el sistema DEBE desplegar una notificación breve y no invasiva informando del resultado.

---

## Edge Cases

- **Intento de exportar selección nula**: Si el usuario presiona `Enter` o `Ctrl+S` antes de demarcar un área válida (estado `Idle`), el sistema NO DEBE ejecutar ninguna acción ni provocar errores.
- **Ruta de disco de solo lectura o espacio insuficiente**: Si el disco de destino no tiene permisos de escritura o carece de espacio disponible, el sistema DEBE atrapar el error de I/O y notificar al usuario sin crashear.
- **Portapapeles del SO bloqueado por otra aplicación (ej. gestor de contraseñas)**: El driver de portapapeles DEBE implementar un reintento no bloqueante (hasta 3 intentos con 50ms de pausa) antes de reportar falla.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-014**: El sistema DEBE permitir copiar la captura al portapapeles del sistema operativo de manera inmediata mediante la tecla `Enter` o el botón "Copiar" de la barra flotante.
- **FR-015**: El sistema DEBE escribir la captura en el portapapeles en formato MIME estándar `image/png`, asegurando compatibilidad multiplataforma con Slack, Microsoft Teams, WhatsApp, Jira y navegadores web.
- **FR-017**: El sistema DEBE soportar el guardado directo en el disco local mediante atajo de teclado (`Ctrl+S` / `Cmd+S`) o botón "Guardar".
- **FR-018**: El sistema DEBE generar nombres de archivo predeterminados con el patrón `Captura_YYYY-MM-DD_HH-mm-ss.[ext]` basándose en el reloj local del sistema.
- **FR-019**: El sistema DEBE admitir codificación en formatos PNG (sin pérdida), JPEG (calidad ajustable) y WebP.
- **FR-020**: El sistema DEBE cerrar inmediatamente la ventana superpuesta (overlay) una vez completada la copia o el guardado.

### Non-Functional & Security Requirements (Constitution)

- **BR-001 / BR-004**: Garantía estricta de privacidad: Toda la composición y exportación de imágenes DEBE realizarse 100% en la memoria local y disco local del dispositivo, con CERO transferencias por red.
- **TR-006**: Escritura atómica y libre de fugas de memoria en buffers de portapapeles y disco.
- **CON-001 (TDD)**: Los casos de uso de exportación, composición de capas y generadores de nomenclatura DEBEN probarse unitariamente de forma aislada con mocks y buffers sintéticos.
- **CON-002 (Clean Architecture)**: Los casos de uso `ExportCaptureToClipboardUseCase` y `ExportCaptureToFileUseCase` residen en la capa de aplicación y dependen exclusivamente de los traits `IClipboardDriver` e `IFileSystemDriver`.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La transferencia al portapapeles se completa en menos de 100 milisegundos tras pulsar `Enter` para imágenes de hasta 4K de resolución.
- **SC-002**: El archivo guardado en disco contiene los píxeles idénticos a los seleccionados con las ofuscaciones y anotaciones superpuestas de manera pixel-perfect.
- **SC-003**: Cero llamadas de red durante todo el proceso de exportación (100% Local-First verificado).
- **SC-004**: Cobertura de pruebas unitarias superior al 95% en los casos de uso y generadores de salida.

---

## Key Entities

- **ImageFileFormat** (Enum): `Png`, `Jpeg { quality: u8 }`, `Webp`.
- **ExportPayload**: Estructura que transporta el buffer crudo o imagen codificada y metadatos de resolución.
- **FileNameGenerator** (Value Object / Helper): Genera nombres de archivo seguros normalizados con marca de tiempo local ISO-8601 compatible con sistemas de archivos.