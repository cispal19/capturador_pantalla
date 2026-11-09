# Feature Specification: 006-settings-and-user-preferences

**Feature Branch**: `006-settings-and-user-preferences`  
**Created**: 2026-11-09  
**Status**: Draft  
**Input**: "Crea la especificación para el panel de configuración y preferencias de usuario conforme a FR-017, FR-018, BR-005, TR-008 y la Constitución."

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Persistencia y Gestión del Esquema de Preferencias Locales (Priority: P1) 🎯 MVP

El usuario necesita que la aplicación recuerde sus configuraciones de trabajo entre reinicios (atajos, carpetas, formato de imagen y preferencias de notificación) de forma totalmente transparente, local y sin necesidad de cuentas ni privilegios administrativos.

**Why this priority**: Es la base del sistema de preferencias; sin almacenamiento local confiable (`config.json`), ninguna personalización puede persistir.

**Independent Test**: Modificar y guardar preferencias, reiniciar el proceso de la aplicación o simular la recarga, y verificar que los valores personalizados se cargan intactos desde el archivo local sin intervención manual.

**Acceptance Scenarios**:
1. **Given** que el usuario ejecuta la aplicación por primera vez (no existe `config.json`), **When** se inicializa el backend, **Then** se crea automáticamente el archivo en la ruta estándar del usuario (`~/.config/capturador-pantalla/config.json` en Unix o `%APPDATA%\capturador-pantalla\config.json` en Windows) con valores seguros por defecto.
2. **Given** un archivo `config.json` corrupto o con JSON malformado, **When** la aplicación arranca, **Then** se registra una advertencia, se cargan los valores predeterminados de fábrica y se regenera el archivo sin crashear.
3. **Given** que el usuario actualiza una preferencia válida mediante la interfaz, **When** se invoca el guardado, **Then** el archivo se escribe en disco de forma atómica y consistente.

---

### User Story 2 - Personalización y Re-registro Dinámico de Atajos Globales (Priority: P1) 🎯 MVP

El usuario desea configurar una combinación de teclas personalizada para disparar la captura de pantalla (por ejemplo, adaptar `Cmd+Shift+X` a `Cmd+Shift+4` o `F13`), y que dicho atajo tenga efecto de inmediato sin tener que reiniciar la aplicación.

**Why this priority**: Los usuarios corporativos tienen flujos de trabajo muy personalizados y suelen experimentar conflictos con combinaciones de teclas reservadas por sus herramientas de trabajo (Slack, IDEs, etc.).

**Independent Test**: Abrir preferencias, cambiar la combinación de teclas de captura, guardar el cambio, presionar el nuevo atajo y comprobar que la sesión interactiva de captura se dispara de inmediato mientras que el atajo anterior queda liberado.

**Acceptance Scenarios**:
1. **Given** que el atajo activo es `CommandOrControl+Shift+X`, **When** el usuario especifica una nueva combinación válida (ej. `CommandOrControl+Shift+C`) y guarda, **Then** el sistema desregistra el atajo anterior, registra el nuevo inmediatamente en el SO y emite confirmación de éxito.
2. **Given** que el usuario introduce una combinación no válida o vacía, **When** intenta guardar, **Then** el sistema rechaza la operación, preserva el atajo existente y muestra un mensaje de error explicativo.
3. **Given** que el atajo solicitado ya está capturado o reservado por el sistema operativo, **When** el driver de atajos falla al registrar, **Then** se restaura el atajo previo y se notifica al usuario del conflicto.

---

### User Story 3 - Configuración de Almacenamiento Local y Formatos (Priority: P2)

El usuario desea definir su carpeta preferida para el guardado automático de capturas (por ejemplo, una carpeta de proyecto o `~/Pictures/Capturas`) y el formato predeterminado de salida (PNG sin pérdidas o JPEG ligero).

**Why this priority**: Evita la fricción de tener que mover capturas manualmente o convertirlas de formato después de guardarlas.

**Independent Test**: Configurar una ruta de destino personalizada y formato JPEG, tomar una captura, presionar `Cmd+S`, y verificar que el archivo resultante se guarda en la carpeta seleccionada en formato `.jpeg`.

**Acceptance Scenarios**:
1. **Given** que el usuario selecciona una carpeta personalizada existente en el disco, **When** guarda las preferencias, **Then** el nuevo valor de `default_save_directory` queda persistido.
2. **Given** que el usuario selecciona `jpeg` como formato predeterminado, **When** realiza una captura y presiona guardar, **Then** el archivo se codifica y nombra con extensión `.jpeg`.
3. **Given** que la carpeta configurada fue borrada o no tiene permisos de escritura, **When** se intenta guardar una captura, **Then** la aplicación recurre automáticamente al directorio Pictures del usuario (fallback) y muestra una advertencia.

---

### User Story 4 - Acceso desde System Tray e Inicio con el Sistema (Autostart) (Priority: P3)

El usuario desea acceder al panel de configuración directamente desde el ícono residente en la barra de tareas / menú de estado ("Preferencias...") y habilitar el inicio automático de la aplicación con el sistema operativo sin requerir permisos de administrador.

**Why this priority**: Mejora la ergonomía en el uso diario para que la herramienta esté siempre disponible en segundo plano.

**Independent Test**: Hacer clic derecho en el System Tray, seleccionar "Preferencias...", verificar que la ventana modal de configuración se despliega de inmediato, activar "Iniciar con el sistema" y comprobar el estado.

**Acceptance Scenarios**:
1. **Given** que la aplicación reside en la bandeja del sistema, **When** el usuario hace clic en "Preferencias...", **Then** se abre la ventana/vista de configuración con los valores actuales cargados.
2. **Given** que el usuario activa la opción "Iniciar automáticamente con el sistema", **When** guarda la preferencia, **Then** se registra la tarea de autostart a nivel de usuario en el SO.

---

### Edge Cases

- **Permisos de escritura restringidos en el directorio de usuario**: Si no se puede escribir en la ruta de configuración predeterminada, la aplicación debe operar en modo memoria volátil con valores por defecto y advertir al usuario sin bloquear la captura.
- **Formato JSON parcialmente incompleto (migración)**: Si una versión futura añade nuevos campos a `AppPreferences`, el deserializador (`serde`) debe utilizar valores predeterminados para los campos ausentes sin sobrescribir destructivamente los campos ya existentes.
- **Ruta de guardado personalizada eliminada**: Si la carpeta especificada por el usuario deja de existir, el backend valida la ruta antes de escribir y hace fallback transparente a la carpeta estándar de imágenes del SO.
- **Múltiples ventanas/llamadas concurrentes**: La lectura y escritura en `config.json` debe estar protegida mediante un `RwLock`/`Mutex` en memoria para evitar colisiones o escrituras truncadas.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-018-01**: El backend en Rust DEBE cargar el archivo de configuración `config.json` al arrancar la aplicación desde la ruta estándar del usuario (`~/.config/capturador-pantalla/config.json` en Linux/macOS, `%APPDATA%\capturador-pantalla\config.json` en Windows).
- **FR-018-02**: Si el archivo o su directorio no existen, el sistema DEBE crearlos automáticamente con valores predeterminados seguros (`capture_shortcut: "CommandOrControl+Shift+X"`, `default_image_format: "png"`, `autostart: false`, `copy_to_clipboard: true`, `show_notifications: true`).
- **FR-018-03**: El backend DEBE exponer el comando IPC `get_preferences` que devuelva el objeto completo tipado `AppPreferences`.
- **FR-018-04**: El backend DEBE exponer el comando IPC `update_preferences` que valide y persista atómicamente el objeto `AppPreferences`.
- **FR-018-05**: Al actualizar `capture_shortcut`, el sistema DEBE desregistrar de inmediato el atajo anterior y registrar el nuevo en el driver de atajos globales sin requerir reinicio del proceso.
- **FR-018-06**: El frontend DEBE presentar una interfaz gráfica de configuración limpia y corporativa con campos para: atajo de captura, carpeta de guardado, formato predeterminado, autostart y notificaciones.
- **FR-018-07**: El menú de la bandeja del sistema (System Tray) DEBE conectar la opción "Preferencias..." para abrir la ventana/panel de configuración.
- **FR-018-08**: El sistema DEBE garantizar que toda la configuración resida exclusivamente en el espacio de usuario (Local-First) y NO DEBE requerir privilegios de superusuario (`sudo`/administrador) en ningún momento (BR-005).

### Key Entities

- **`AppPreferences` (Value Object / Entidad de Dominio)**:
  - `capture_shortcut`: Cadena con la combinación de teclas (ej. `"CommandOrControl+Shift+X"` o `"PrintScreen"`).
  - `default_save_directory`: Ruta opcional (`Option<String>`) al directorio donde se guardarán las capturas.
  - `default_image_format`: Formato predeterminado de codificación (`"png"` o `"jpeg"`).
  - `autostart_enabled`: Booleano que indica si la app debe arrancar con el inicio de sesión del usuario.
  - `copy_to_clipboard_on_capture`: Booleano que determina si se copia al portapapeles tras capturar.
  - `show_notifications`: Booleano para activar/desactivar notificaciones visuales toast de confirmación.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La carga y parseo del archivo de configuración toma menos de 10 milisegundos en el inicio de la aplicación.
- **SC-002**: El re-registro del atajo global modificado toma menos de 100 milisegundos desde que el usuario hace clic en "Guardar".
- **SC-003**: 0% de operaciones de configuración requieren privilegios elevados de superusuario/administrador (100% user-space compliance).
- **SC-004**: Cobertura del 100% de pruebas unitarias TDD sobre la serialización, deserialización, fallbacks y validaciones de `AppPreferences`.
- **SC-005**: Tolerancia a fallos: La aplicación arranca de forma 100% funcional incluso si el archivo `config.json` está dañado o inaccesible (resiliencia por diseño).

---

## Assumptions

- El usuario tiene permisos estándar de lectura y escritura en su propio directorio personal (`HOME` o `APPDATA`).
- Tauri provee rutas estándar de configuración mediante APIs del runtime o librerías estándar multiplataforma (`directories` o `std::env`).
- Las combinaciones de teclas utilizan la convención de Tauri / Electron (`CommandOrControl`, `Shift`, `Alt`, `Super`, teclas alfanuméricas o de función).
