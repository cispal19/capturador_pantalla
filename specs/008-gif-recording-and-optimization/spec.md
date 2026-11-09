# Feature Specification: 008-gif-recording-and-optimization

**Feature Branch**: `008-gif-recording-and-optimization`  
**Created**: 2026-11-09  
**Status**: Draft  
**Input**: "Crea la especificación para la exportación y optimización de animaciones GIF conforme a FR-020, BR-001, BR-004 y la Constitución."

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Grabación y Exportación Directa a GIF Animado (Priority: P1) 🎯 MVP

El usuario necesita capturar una secuencia rápida de pantalla (por ejemplo, demostrar un comportamiento de interfaz o un bug) y convertirla inmediatamente en un archivo `.gif` ligero y animado para pegarlo en Slack, Teams o GitHub sin tener que subir videos a servicios externos.

**Why this priority**: Es la necesidad más frecuente de ingenieros de software, diseñadores y soporte técnico que requieren ilustrar interacciones rápidas sin fricción de reproductores de video.

**Independent Test**: Seleccionar una región de pantalla, pulsar "Grabar GIF", interactuar durante 3 segundos, finalizar y verificar que se genera un archivo `.gif` válido con encabezado GIF89a estándar y reproducción fluida en bucle continuo.

**Acceptance Scenarios**:
1. **Given** que el usuario selecciona una región de pantalla y el modo "GIF", **When** pulsa "Grabar GIF" y finaliza la captura, **Then** el sistema procesa los frames acumulados, codifica la animación en formato GIF89a y guarda el archivo en disco local con nomenclatura `Animacion_YYYY-MM-DD_HH-mm-ss.gif`.
2. **Given** una grabación de GIF completada, **When** el usuario pulsa "Copiar al portapapeles", **Then** el archivo o la imagen GIF se copia al portapapeles nativo para pegado directo.
3. **Given** que la grabación alcanza el límite máximo de 15 segundos para GIFs, **When** transcurre el segundo 15, **Then** el sistema finaliza automáticamente la captura y genera el archivo para proteger la memoria y el tamaño final.

---

### User Story 2 - Optimización de Paleta y Control de Tasa de Cuadros (Priority: P2)

El usuario corporativo requiere que el archivo GIF final no sobrepase los límites de tamaño habituales de herramientas corporativas (ej. < 5-10 MB) manteniendo una nitidez visual aceptable.

**Why this priority**: Plataformas como Slack, Jira o Microsoft Teams rechazan o congelan archivos GIF pesados (> 10-15 MB).

**Independent Test**: Grabar una secuencia con movimiento moderado durante 5 segundos a 15 FPS; el archivo resultante debe pesar menos de 5 MB y tener una paleta cuantizada limpia sin banding severo.

**Acceptance Scenarios**:
1. **Given** una secuencia de frames RGBA, **When** se ejecuta el codificador GIF, **Then** se reduce la paleta a 256 colores optimizados por frame utilizando cuantización de color.
2. **Given** frames consecutivos idénticos o con áreas estáticas, **When** se codifica el GIF, **Then** se optimiza la compresión temporal reduciendo el peso final del archivo.

---

### User Story 3 - Vista Previa Rápida y Acciones de Salida (Priority: P3)

El usuario desea visualizar brevemente la animación resultante antes de confirmarla o descartarla, con botones directos para "Copiar", "Guardar" y "Descartar".

**Why this priority**: Evita guardar animaciones erróneas y agiliza el flujo de compartir.

**Independent Test**: Detener la grabación de un GIF; una ventana o tarjeta flotante de vista previa debe mostrar la animación reproduciéndose en bucle, permitiendo guardar o descartar con un clic.

**Acceptance Scenarios**:
1. **Given** un GIF recién codificado, **When** finaliza el procesamiento, **Then** se muestra la vista previa con botones para Guardar en disco o Copiar.
2. **Given** la vista previa activa, **When** el usuario pulsa "Descartar", **Then** el archivo temporal se elimina sin dejar residuos.

---

### Edge Cases

- **Grabación extremadamente corta (< 0.5 segundos o 1 solo frame)**: El codificador debe duplicar el frame o asegurar un retardo mínimo de 100 ms para que el visor de GIF no crashee ni parpadee.
- **Áreas con alto gradiente de color**: La cuantización a 256 colores debe usar dithering para evitar saltos bruscos de tonalidad.
- **Resolución grande (ej. 4K o 1080p completo)**: El motor debe escalar opcionalmente o advertir si el tamaño estimado superará los 15 MB, recomendando reducir el área o usar WebM.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-020-01**: El sistema DEBE ofrecer un modo de exportación "GIF Animado" en la barra de herramientas de selección.
- **FR-020-02**: El backend DEBE implementar el caso de uso `ExportRecordingToGifUseCase` que reciba secuencias de frames RGBA o video sintético y codifique un archivo GIF89a conforme al estándar.
- **FR-020-03**: La tasa de cuadros (FPS) para exportación a GIF DEBE ser configurable entre 10 y 24 FPS (por defecto 15 FPS) para optimizar el tamaño del archivo.
- **FR-020-04**: El límite máximo de duración para una grabación de GIF DEBE ser de 15 segundos (para evitar desbordamiento de memoria).
- **FR-020-05**: El archivo GIF generado DEBE persistirse con la convención `Animacion_YYYY-MM-DD_HH-mm-ss.gif` en la carpeta configurada por el usuario.
- **FR-020-06**: El comando IPC `export_recording_to_gif` DEBE retornar los metadatos completos: ruta absoluta, duración, tamaño en bytes y número de frames.
- **FR-020-07**: El sistema DEBE garantizar el principio Local-First (BR-001): 0% de uso de APIs en la nube o procesamiento externo.

### Key Entities

- **`GifExportConfig` (Value Object)**:
  - `fps`: Tasa de cuadros por segundo (10 a 24, predeterminado 15).
  - `max_duration_seconds`: Límite de tiempo (máximo 15 segundos).
  - `repeat`: Conteo de repeticiones (0 = bucle infinito).
- **`GifExportResult` (Value Object)**:
  - `file_path`: Ruta del archivo GIF generado.
  - `duration_seconds`: Duración de la animación.
  - `file_size_bytes`: Tamaño del archivo GIF en disco.
  - `frame_count`: Cantidad total de fotogramas incluidos.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Codificación de un GIF de 5 segundos a 15 FPS en menos de 1.5 segundos de tiempo de procesamiento local.
- **SC-002**: Tamaño promedio de archivo de un GIF de 5 segundos en resolución estándar < 5 MB.
- **SC-003**: Cero archivos residuales temporales en disco tras cancelar o descartar la exportación.
- **SC-004**: 100% de cobertura de pruebas unitarias TDD sobre la estructura del archivo GIF89a, cuantización, configuración y comandos IPC.

---

## Assumptions

- El formato GIF89a con bucle infinito (`Netscape 2.0 loop extension`) es soportado por todas las plataformas de mensajería corporativa.
- El crate `image` de Rust dispone de la feature `gif` integrada para codificación de streams de fotogramas con retardo por frame.
