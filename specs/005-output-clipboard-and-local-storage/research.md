# Research: Clipboard Pipeline & Local Storage Drivers

**Feature**: `005-output-clipboard-and-local-storage`  
**Date**: 2026-11-09  
**Status**: Completed

---

## 1. Portapapeles Multiplataforma Nativo (`arboard`)

### Decisión
Utilizar la librería `arboard` (v3.4) para el adaptador `ArboardClipboardDriver` implementando `IClipboardDriver`.

### Rationale
- **Zero Temporary Files**: `arboard` soporta escribir directamente buffers RGBA en memoria viva del portapapeles del sistema operativo (`ImageData { width, height, bytes }`) sin crear archivos temporales en el disco duro (`/tmp`). Esto reduce la latencia a < 25ms y elimina riesgos de persistencia no deseada de datos sensibles.
- **Licencia y Mantenimiento Corporativo**: Licencia dual MIT / Apache-2.0, activamente mantenida, ampliamente probada en macOS (Pasteboard), Windows (Win32 clipboard) y Linux (X11 / Wayland).
- **MIME `image/png`**: Los sistemas operativos convierten internamente la estructura `ImageData` en los formatos estándar reconocidos por Slack, Microsoft Teams, Jira, WhatsApp y clientes de correo al pegar.

---

## 2. Codificación y Almacenamiento Local (`image` Crate)

### Decisión
Utilizar el crate `image` (v0.25) para codificar buffers RGBA en memoria hacia formatos de archivo estándar:
- **PNG**: Sin pérdida (`image::codecs::png::PngEncoder`).
- **JPEG**: Con calidad configurable 0-100 (`image::codecs::jpeg::JpegEncoder`).
- **WebP**: Compresión moderna optimizada.

### Nomenclatura Automática de Archivos
Generación de nombre ordenable cronológicamente:
`Captura_YYYY-MM-DD_HH-mm-ss.[ext]`
Garantiza unicidad y ordenamiento natural en el explorador de archivos sin colisiones.

---

## 3. Composición en Memoria (Base Screen + Blur + Annotations)

### Decisión
- **En el Frontend**: La transferencia rápida puede utilizar el lienzo de HTML5 Canvas (`canvas.toBlob('image/png')` o `canvas.toDataURL()`), transfiriendo los bytes codificados al backend o utilizando el portapapeles nativo web (`navigator.clipboard.write([new ClipboardItem(...)])`) con fallback automático al comando IPC de Rust si el contexto del sistema lo requiere.
- **En el Backend**: El comando IPC `copy_capture_to_clipboard` recibe las dimensiones del recorte y los bytes RGBA / PNG, ejecutando la copia atómica.

---

## 4. Manejo de Errores y Privacidad

- **Bloqueo del Portapapeles**: Si el portapapeles está bloqueado por otra aplicación, `ArboardClipboardDriver` reintenta hasta 3 veces con una pausa de 50ms antes de fallar.
- **Local-First Estricto**: 0 llamadas de red, 0 telemetría, 0 almacenamiento en servidores remotos.