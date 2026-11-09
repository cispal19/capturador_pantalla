# Ports & Driver Interface Contracts

**Feature**: `001-scaffolding-clean-architecture`  
**Date**: 2026-11-09  
**Status**: Formal Specification  

These interfaces represent abstract ports in `src-tauri/src/ports/`. They define the boundary between the application use cases and external infrastructure adapters in accordance with the Dependency Inversion Principle (SOLID DIP).

---

## 1. Screen Capture Port (`IScreenCaptureDriver`)

```rust
use crate::domain::{RawDisplayBuffer, DomainError};

#[derive(Debug, thiserror::Error)]
pub enum CaptureDriverError {
    #[error("Permission denied to record screen")]
    PermissionDenied,
    #[error("No active display detected")]
    NoDisplayFound,
    #[error("Internal capture error: {0}")]
    Internal(String),
}

pub trait IScreenCaptureDriver: Send + Sync {
    /// Captures the primary or active display into raw memory
    fn capture_active_display(&self) -> Result<RawDisplayBuffer, CaptureDriverError>;

    /// Captures all connected displays (for multi-monitor setups)
    fn capture_all_displays(&self) -> Result<Vec<RawDisplayBuffer>, CaptureDriverError>;
}
```

---

## 2. Clipboard Port (`IClipboardDriver`)

```rust
use crate::domain::RawDisplayBuffer;

#[derive(Debug, thiserror::Error)]
pub enum ClipboardDriverError {
    #[error("Failed to access system clipboard: {0}")]
    AccessFailed(String),
    #[error("Unsupported image format for clipboard")]
    UnsupportedFormat,
}

pub trait IClipboardDriver: Send + Sync {
    /// Writes raw pixel buffer as an image directly to the OS clipboard
    fn copy_image(&self, buffer: &RawDisplayBuffer) -> Result<(), ClipboardDriverError>;

    /// Clears the clipboard
    fn clear(&self) -> Result<(), ClipboardDriverError>;
}
```

---

## 3. File System Port (`IFileSystemDriver`)

```rust
use std::path::{Path, PathBuf};
use crate::domain::RawDisplayBuffer;

#[derive(Debug, thiserror::Error)]
pub enum FileSystemDriverError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Encoding error: {0}")]
    Encoding(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFileFormat {
    Png,
    Jpeg { quality: u8 },
    Webp,
}

pub trait IFileSystemDriver: Send + Sync {
    /// Encodes and writes a raw display buffer to a local file path
    fn save_image(
        &self,
        path: &Path,
        buffer: &RawDisplayBuffer,
        format: ImageFileFormat,
    ) -> Result<PathBuf, FileSystemDriverError>;

    /// Reads configuration JSON file from disk
    fn read_text_file(&self, path: &Path) -> Result<String, FileSystemDriverError>;

    /// Writes text file atomically to disk
    fn write_text_file(&self, path: &Path, content: &str) -> Result<(), FileSystemDriverError>;
}
```

---

## 4. Global Shortcut Port (`IGlobalShortcutDriver`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum ShortcutDriverError {
    #[error("Shortcut combination already registered: {0}")]
    AlreadyRegistered(String),
    #[error("Invalid accelerator syntax: {0}")]
    InvalidSyntax(String),
    #[error("System registration failed: {0}")]
    SystemError(String),
}

pub trait IGlobalShortcutDriver: Send + Sync {
    /// Registers a global hotkey with a trigger callback
    fn register_shortcut(
        &self,
        accelerator: &str,
        callback: Box<dyn Fn() + Send + 'static>,
    ) -> Result<(), ShortcutDriverError>;

    /// Unregisters all registered hotkeys
    fn unregister_all(&self) -> Result<(), ShortcutDriverError>;
}
```
