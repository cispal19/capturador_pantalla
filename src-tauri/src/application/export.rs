use crate::domain::buffer::RawDisplayBuffer;
use crate::ports::{
    ClipboardDriverError, FileSystemDriverError, IClipboardDriver, IFileSystemDriver,
    ImageFileFormat,
};
use std::path::{Path, PathBuf};

/// Use case: Exports a capture buffer directly to the OS clipboard
pub struct ExportToClipboardUseCase<C: IClipboardDriver> {
    clipboard_driver: C,
}

impl<C: IClipboardDriver> ExportToClipboardUseCase<C> {
    pub fn new(clipboard_driver: C) -> Self {
        Self { clipboard_driver }
    }

    pub fn execute(&self, buffer: &RawDisplayBuffer) -> Result<(), ClipboardDriverError> {
        self.clipboard_driver.copy_image(buffer)
    }
}

/// Use case: Exports a capture buffer to a local image file
pub struct ExportToFileUseCase<F: IFileSystemDriver> {
    file_system_driver: F,
}

impl<F: IFileSystemDriver> ExportToFileUseCase<F> {
    pub fn new(file_system_driver: F) -> Self {
        Self { file_system_driver }
    }

    pub fn execute(
        &self,
        buffer: &RawDisplayBuffer,
        path: &Path,
        format: ImageFileFormat,
    ) -> Result<PathBuf, FileSystemDriverError> {
        self.file_system_driver.save_image(path, buffer, format)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::infrastructure::{FileSystemStorageDriver, MockClipboardDriver};
    use std::env;
    use std::fs;

    #[test]
    fn test_export_to_clipboard_use_case() {
        let mock_driver = MockClipboardDriver::new();
        let use_case = ExportToClipboardUseCase::new(mock_driver.clone());

        let buffer = RawDisplayBuffer::new(
            20,
            20,
            80,
            crate::domain::buffer::PixelFormat::Rgba8,
            vec![128; 1600],
        )
        .unwrap();

        assert!(use_case.execute(&buffer).is_ok());
        assert!(mock_driver.get_last_copied().is_some());
    }

    #[test]
    fn test_export_to_file_use_case() {
        let fs_driver = FileSystemStorageDriver::new();
        let use_case = ExportToFileUseCase::new(fs_driver);

        let buffer = RawDisplayBuffer::new(
            20,
            20,
            80,
            crate::domain::buffer::PixelFormat::Rgba8,
            vec![128; 1600],
        )
        .unwrap();

        let temp_dir = env::temp_dir();
        let out_path = temp_dir.join("test_export_use_case.png");

        let saved = use_case
            .execute(&buffer, &out_path, ImageFileFormat::Png)
            .unwrap();
        assert!(saved.exists());
        let _ = fs::remove_file(saved);
    }
}
