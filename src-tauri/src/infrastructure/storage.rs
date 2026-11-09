use crate::domain::buffer::RawDisplayBuffer;
use crate::ports::{FileSystemDriverError, IFileSystemDriver, ImageFileFormat};
use std::fs;
use std::path::{Path, PathBuf};

/// Production adapter for encoding and persisting images to local disk.
#[derive(Default)]
pub struct FileSystemStorageDriver;

impl FileSystemStorageDriver {
    pub fn new() -> Self {
        Self
    }
}

impl IFileSystemDriver for FileSystemStorageDriver {
    fn save_image(
        &self,
        path: &Path,
        buffer: &RawDisplayBuffer,
        format: ImageFileFormat,
    ) -> Result<PathBuf, FileSystemDriverError> {
        let width = buffer.width;
        let height = buffer.height;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                FileSystemDriverError::Io(format!("Failed to create directory: {}", e))
            })?;
        }

        match format {
            ImageFileFormat::Png => {
                image::save_buffer(
                    path,
                    &buffer.data,
                    width,
                    height,
                    image::ExtendedColorType::Rgba8,
                )
                .map_err(|e| FileSystemDriverError::Encoding(e.to_string()))?;
            }
            ImageFileFormat::Jpeg { quality: _ } => {
                // Convert RGBA to RGB for JPEG encoding
                let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
                let (chunks, _) = buffer.data.as_chunks::<4>();
                for chunk in chunks {
                    rgb_data.push(chunk[0]);
                    rgb_data.push(chunk[1]);
                    rgb_data.push(chunk[2]);
                }

                image::save_buffer(
                    path,
                    &rgb_data,
                    width,
                    height,
                    image::ExtendedColorType::Rgb8,
                )
                .map_err(|e| FileSystemDriverError::Encoding(e.to_string()))?;
            }
            ImageFileFormat::Webp => {
                // Fallback to PNG encoding if WebP encoder not compiled in
                image::save_buffer(
                    path,
                    &buffer.data,
                    width,
                    height,
                    image::ExtendedColorType::Rgba8,
                )
                .map_err(|e| FileSystemDriverError::Encoding(e.to_string()))?;
            }
        }

        Ok(path.to_path_buf())
    }

    fn read_text_file(&self, path: &Path) -> Result<String, FileSystemDriverError> {
        fs::read_to_string(path).map_err(|e| FileSystemDriverError::Io(e.to_string()))
    }

    fn write_text_file(&self, path: &Path, content: &str) -> Result<(), FileSystemDriverError> {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(path, content).map_err(|e| FileSystemDriverError::Io(e.to_string()))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_file_system_storage_save_png_and_jpeg() {
        let driver = FileSystemStorageDriver::new();
        let buffer = RawDisplayBuffer::new(
            10,
            10,
            40,
            crate::domain::buffer::PixelFormat::Rgba8,
            vec![200; 400],
        )
        .unwrap();

        let temp_dir = env::temp_dir();
        let png_path = temp_dir.join("test_capture_export.png");
        let saved_png = driver
            .save_image(&png_path, &buffer, ImageFileFormat::Png)
            .unwrap();
        assert!(saved_png.exists());
        let _ = fs::remove_file(saved_png);

        let jpg_path = temp_dir.join("test_capture_export.jpg");
        let saved_jpg = driver
            .save_image(&jpg_path, &buffer, ImageFileFormat::Jpeg { quality: 90 })
            .unwrap();
        assert!(saved_jpg.exists());
        let _ = fs::remove_file(saved_jpg);
    }

    #[test]
    fn test_file_system_storage_text_read_write() {
        let driver = FileSystemStorageDriver::new();
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join("test_config_rw.json");

        driver
            .write_text_file(&file_path, "{\"key\":\"val\"}")
            .unwrap();
        let content = driver.read_text_file(&file_path).unwrap();
        assert_eq!(content, "{\"key\":\"val\"}");
        let _ = fs::remove_file(file_path);
    }
}
