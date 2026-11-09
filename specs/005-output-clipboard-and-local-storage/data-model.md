# Data Model: Output Pipeline & Storage Primitives

**Feature**: `005-output-clipboard-and-local-storage`  
**Date**: 2026-11-09  
**Status**: Formal Data Model

---

## 1. Domain Entities & Value Objects (Rust Core)

### `FileNameGenerator` (Domain Helper)
```rust
pub struct FileNameGenerator;

impl FileNameGenerator {
    /// Generates timestamped filename: Captura_YYYY-MM-DD_HH-mm-ss.{ext}
    pub fn generate_name(format: &str, timestamp_secs: u64) -> String;
}
```

### `ExportCapturePayload` (DTO)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCapturePayload {
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u8>,
}
```

### `SaveImageResult` (Response DTO)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveImageResult {
    pub file_path: String,
    pub file_size_bytes: usize,
    pub format: String,
}
```

---

## 2. Ports & Adapters Models

### `MockClipboardDriver` (Infrastructure Test Double)
```rust
pub struct MockClipboardDriver {
    pub stored_buffer: std::sync::Arc<std::sync::Mutex<Option<RawDisplayBuffer>>>,
}
```

### `ArboardClipboardDriver` (Production Adapter)
```rust
pub struct ArboardClipboardDriver;

impl IClipboardDriver for ArboardClipboardDriver {
    fn copy_image(&self, buffer: &RawDisplayBuffer) -> Result<(), ClipboardDriverError>;
    fn clear(&self) -> Result<(), ClipboardDriverError>;
}
```

### `FileSystemStorageDriver` (Production Adapter)
```rust
pub struct FileSystemStorageDriver;

impl IFileSystemDriver for FileSystemStorageDriver {
    fn save_image(
        &self,
        path: &Path,
        buffer: &RawDisplayBuffer,
        format: ImageFileFormat,
    ) -> Result<PathBuf, FileSystemDriverError>;
    fn read_text_file(&self, path: &Path) -> Result<String, FileSystemDriverError>;
    fn write_text_file(&self, path: &Path, content: &str) -> Result<(), FileSystemDriverError>;
}
```

---

## 3. Mathematical & Formatting Invariants

| ID Regla | Expresión | Descripción |
| :--- | :--- | :--- |
| **INV-NAME-01** | `Captura_\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}\.(png\|jpg\|webp)` | El nombre de archivo siempre respeta la expresión regular de fecha/hora ISO. |
| **INV-BUF-01** | `rgba_data.len() == (width * height * 4) as usize` | La longitud del buffer transferido coincide exactamente con los 4 bytes de canal RGBA por píxel. |
| **INV-RET-01** | `retries <= 3` | Los reintentos de portapapeles no exceden 3 iteraciones para evitar bloqueos del hilo principal. |