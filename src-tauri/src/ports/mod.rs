use crate::domain::buffer::RawDisplayBuffer;
use std::path::{Path, PathBuf};

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
