# Feature Specification: 007-screen-recording-engine

**Feature Branch**: `007-screen-recording-engine`  
**Created**: 2026-11-09  
**Status**: Draft  
**Input**: "Crea la especificación para la funcionalidad de grabación de video en región de pantalla conforme a FR-019, BR-001, BR-007 y la Constitución."

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Grabación de Video en Región Seleccionada (Priority: P1) 🎯 MVP

El usuario necesita delimitar un área específica de la pantalla y grabar video continuo (MP4/WebM) de lo que ocurre en ella (por ejemplo, reproducir un bug o documentar un procedimiento) con controles claros de Iniciar, Detener y Cancelar.

**Why this priority**: Es la funcionalidad troncal de la Fase 2; permite registrar secuencias de movimiento corporativas sin necesidad de software pesado de terceros.

**Independent Test**: Seleccionar un área en pantalla, pulsar "Grabar Video", interactuar durante unos segundos, pulsar "Detener", y verificar que se genera un archivo de video local válido (`.webm` o `.mp4`) en el directorio de capturas con la duración y dimensiones exactas.

**Acceptance Scenarios**:
1. **Given** que el usuario selecciona una región válida (ancho >= 50px, alto >= 50px), **When** pulsa "Grabar Video", **Then** el sistema inicia la sesión de grabación, proyecta el indicador de grabación (borde rojo activo) y comienza a contabilizar el tiempo.
2. **Given** una grabación en curso, **When** el usuario pulsa "Detener", **Then** el motor finaliza el empaquetado del video, guarda el archivo en disco local con nomenclatura ISO (`Grabacion_YYYY-MM-DD_HH-mm-ss.[ext]`) y muestra una notificación toast de confirmación.
3. **Given** una grabación en curso, **When** el usuario pulsa "Cancelar" o presiona `Esc`, **Then** la grabación se aborta inmediatamente, los recursos se liberan sin dejar archivos residuales corruptos y el estado vuelve a `Idle`.

---

### User Story 2 - Controles Flotantes y Pausa/Reanudación (Priority: P2)

El usuario necesita pausar temporalmente la captura de video mientras prepara otra ventana o introduce credenciales sensibles fuera de cámara, para luego reanudar la grabación sin generar dos archivos separados.

**Why this priority**: Evita tener que editar o concatenar videos posteriormente, ahorrando tiempo y reduciendo riesgos de seguridad.

**Independent Test**: Iniciar grabación, pulsar "Pausar", verificar que el temporizador se congela y no se capturan frames, pulsar "Reanudar", continuar y finalizar; el video resultante debe ser continuo.

**Acceptance Scenarios**:
1. **Given** una grabación en estado `Recording`, **When** el usuario pulsa "Pausar", **Then** el estado pasa a `Paused`, el temporizador se detiene y el borde de grabación cambia visualmente a color ámbar/amarillo.
2. **Given** una grabación en estado `Paused`, **When** el usuario pulsa "Reanudar", **Then** el estado regresa a `Recording`, el temporizador continúa acumulando tiempo y el borde vuelve a rojo pulsante.

---

### User Story 3 - Límite de Tiempo y Protección de Almacenamiento (Priority: P3)

El usuario corporativo requiere salvaguardas para no agotar el espacio en disco ni sobrecargar la CPU si olvida una grabación encendida.

**Why this priority**: Estabilidad del sistema y políticas de gobernanza empresarial (BR-001, BR-007).

**Independent Test**: Configurar un límite máximo (ej. 300 segundos / 5 minutos) o probar el disparo del límite; el sistema debe autodetener la grabación y salvar el archivo de forma íntegra.

**Acceptance Scenarios**:
1. **Given** una grabación que alcanza el tiempo máximo configurado (por defecto 10 minutos), **When** transcurre el último segundo, **Then** el sistema detiene automáticamente la captura, guarda el archivo y emite un toast informativo: "⏱️ Tiempo máximo de grabación alcanzado. Video guardado".

---

### Edge Cases

- **Resolución impar o no múltiplo de 2**: Ciertas especificaciones de video (ej. H.264 / VP9) exigen dimensiones pares. El sistema debe normalizar o redondear automáticamente el ancho y alto a números pares (`width & !1`).
- **Espacio en disco insuficiente**: Si el almacenamiento libre cae por debajo de un umbral de seguridad (ej. < 100 MB), la grabación debe finalizarse de emergencia preservando los frames capturados hasta ese instante.
- **Cierre inesperado de la aplicación o apagado**: El contenedor de video debe estar estructurado en segmentos o finalizar limpiamente en el hook de terminación para no generar archivos ilegibles.
- **Uso de CPU / tasa de refresco**: El muestreo debe estar calibrado a 30 FPS estables por defecto para estaciones corporativas estándar sin saturar el hilo principal.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-019-01**: El sistema DEBE permitir alternar entre el modo "Captura Estática" y el modo "Grabación de Video" desde la interfaz del selector de región.
- **FR-019-02**: El backend DEBE implementar el caso de uso `StartScreenRecordingUseCase` que inicialice la captura continua de frames sobre la región delimitada (`x`, `y`, `width`, `height`).
- **FR-019-03**: El backend DEBE implementar `PauseScreenRecordingUseCase`, `ResumeScreenRecordingUseCase`, `StopScreenRecordingUseCase` y `CancelScreenRecordingUseCase`.
- **FR-019-04**: El sistema DEBE exponer el estado de grabación mediante el comando IPC `get_recording_status` y emitir eventos de progreso temporal (`recording-tick`) cada segundo.
- **FR-019-05**: El frontend DEBE desplegar una barra flotante de control (`RecordingToolbar`) anclada a la región con temporizador en formato `MM:SS`, botones de Iniciar/Pausar, Detener y Cancelar.
- **FR-019-06**: Durante la grabación activa, el área de la pantalla DEBE estar delimitada por un borde visual de color rojo pulsante (o ámbar cuando esté en pausa) para informar al usuario de que la pantalla está siendo grabada.
- **FR-019-07**: El video final DEBE codificarse localmente en formatos estándar (`WebM` o `MP4`) y guardarse en la carpeta predeterminada del usuario con la nomenclatura `Grabacion_YYYY-MM-DD_HH-mm-ss.[ext]`.
- **FR-019-08**: Cumplimiento estricto de Local-First (BR-001): 0% de transmisiones o codificaciones en la nube; todo el flujo ocurre en memoria y almacenamiento local.

### Key Entities

- **`RecordingState` (Enum de Dominio)**:
  - `Idle`: Sin sesión activa de grabación.
  - `Recording`: Adquisición activa de frames y temporizador en marcha.
  - `Paused`: Adquisición temporalmente detenida; el temporizador no avanza.
  - `Finalizing`: Empaquetando y escribiendo contenedor final a disco.
- **`RecordingRegion` (Value Object)**:
  - `x`: Posición horizontal física.
  - `y`: Posición vertical física.
  - `width`: Ancho en píxeles (normalizado a múltiplo de 2).
  - `height`: Alto en píxeles (normalizado a múltiplo de 2).
  - `fps`: Tasa de muestreo (por defecto 30 FPS).
  - `time_limit_seconds`: Límite máximo en segundos (por defecto 600s = 10 min).
- **`RecordingMetadata` (Value Object)**:
  - `file_path`: Ruta absoluta del video guardado.
  - `duration_seconds`: Duración efectiva de la grabación.
  - `file_size_bytes`: Tamaño del archivo en bytes.
  - `format`: Formato de video (`"webm"` o `"mp4"`).
  - `fps`: Fotogramas por segundo del video final.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Latencia de respuesta para Iniciar / Pausar / Reanudar grabación < 100 ms tras hacer clic.
- **SC-002**: Tasa de cuadros estable a 30 FPS constantes durante la captura en resoluciones estándar (1080p).
- **SC-003**: Cero archivos residuales o buffers huérfanos en memoria si el usuario cancela la grabación.
- **SC-004**: 100% de cobertura de pruebas unitarias TDD sobre la máquina de estados, normalización de dimensiones pares, temporizadores y comandos IPC.

---

## Assumptions

- El hardware de la estación de trabajo dispone de suficiente capacidad para codificación de video en tiempo real a 30 FPS.
- Los navegadores y visores de video corporativos soportan nativamente WebM (VP8/VP9) y MP4 (H.264).
