use crate::domain::{
    buffer::RawDisplayBuffer,
    display::DisplayInfo,
    recording::{RecordingMetadata, RecordingRegion, RecordingState},
};
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum CaptureDriverError {
    #[error("Permission denied to record screen")]
    PermissionDenied,
    #[error("No active display detected")]
    NoDisplayFound,
    #[error("Display with id {0} not found")]
    DisplayNotFound(u32),
    #[error("Internal capture error: {0}")]
    Internal(String),
}

pub trait IScreenCaptureDriver: Send + Sync {
    /// Enumerate all connected physical displays
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, CaptureDriverError>;

    /// Captures the primary or active display into raw memory
    fn capture_active_display(&self) -> Result<RawDisplayBuffer, CaptureDriverError>;

    /// Capture raw pixel buffer from a specific display by its ID
    fn capture_display_by_id(
        &self,
        display_id: u32,
    ) -> Result<RawDisplayBuffer, CaptureDriverError>;

    /// Captures all connected displays (for multi-monitor setups)
    fn capture_all_displays(
        &self,
    ) -> Result<Vec<(DisplayInfo, RawDisplayBuffer)>, CaptureDriverError>;
}

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
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

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum FileSystemDriverError {
    #[error("IO error: {0}")]
    Io(String),
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

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
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
        callback: Box<dyn Fn() + Send + Sync + 'static>,
    ) -> Result<(), ShortcutDriverError>;

    /// Check if shortcut is currently active
    fn is_registered(&self, accelerator: &str) -> bool;

    /// Unregisters all registered hotkeys
    fn unregister_all(&self) -> Result<(), ShortcutDriverError>;

    /// Unregisters a specific hotkey
    fn unregister_shortcut(&self, accelerator: &str) -> Result<(), ShortcutDriverError> {
        let _ = accelerator;
        self.unregister_all()
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum RecorderDriverError {
    #[error("No active recording session")]
    NoActiveSession,
    #[error("Recording already in progress")]
    AlreadyRecording,
    #[error("Invalid state transition from {0:?}")]
    InvalidState(RecordingState),
    #[error("Encoding or IO error: {0}")]
    EncodingError(String),
}

pub trait IScreenRecorderDriver: Send + Sync {
    /// Start recording the given region
    fn start(&self, region: &RecordingRegion) -> Result<(), RecorderDriverError>;

    /// Pause the active recording
    fn pause(&self) -> Result<(), RecorderDriverError>;

    /// Resume the paused recording
    fn resume(&self) -> Result<(), RecorderDriverError>;

    /// Stop recording and finalize video output
    fn stop(
        &self,
        video_bytes: &[u8],
        duration_seconds: u32,
        format: &str,
    ) -> Result<RecordingMetadata, RecorderDriverError>;

    /// Cancel and discard the current recording
    fn cancel(&self) -> Result<(), RecorderDriverError>;

    /// Get current state
    fn state(&self) -> RecordingState;
}
