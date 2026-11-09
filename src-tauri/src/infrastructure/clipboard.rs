use crate::domain::buffer::RawDisplayBuffer;
use crate::ports::{ClipboardDriverError, IClipboardDriver};
use arboard::{Clipboard, ImageData};
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

/// Production adapter writing directly to the operating system clipboard using `arboard`.
#[derive(Default)]
pub struct ArboardClipboardDriver;

impl ArboardClipboardDriver {
    pub fn new() -> Self {
        Self
    }
}

impl IClipboardDriver for ArboardClipboardDriver {
    fn copy_image(&self, buffer: &RawDisplayBuffer) -> Result<(), ClipboardDriverError> {
        let mut clipboard =
            Clipboard::new().map_err(|e| ClipboardDriverError::AccessFailed(e.to_string()))?;

        let img_data = ImageData {
            width: buffer.width as usize,
            height: buffer.height as usize,
            bytes: Cow::Borrowed(&buffer.data),
        };

        // Retry up to 3 times in case clipboard is temporarily held by another process
        let mut last_err = None;
        for _ in 0..3 {
            match clipboard.set_image(img_data.clone()) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    last_err = Some(e);
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }
        }

        Err(ClipboardDriverError::AccessFailed(
            last_err.map(|e| e.to_string()).unwrap_or_default(),
        ))
    }

    fn clear(&self) -> Result<(), ClipboardDriverError> {
        let mut clipboard =
            Clipboard::new().map_err(|e| ClipboardDriverError::AccessFailed(e.to_string()))?;
        clipboard
            .clear()
            .map_err(|e| ClipboardDriverError::AccessFailed(e.to_string()))
    }
}

/// Headless test double storing copied buffers in memory.
#[derive(Default, Clone)]
pub struct MockClipboardDriver {
    pub stored_buffer: Arc<Mutex<Option<RawDisplayBuffer>>>,
}

impl MockClipboardDriver {
    pub fn new() -> Self {
        Self {
            stored_buffer: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_last_copied(&self) -> Option<RawDisplayBuffer> {
        self.stored_buffer.lock().unwrap().clone()
    }
}

impl IClipboardDriver for MockClipboardDriver {
    fn copy_image(&self, buffer: &RawDisplayBuffer) -> Result<(), ClipboardDriverError> {
        let mut lock = self.stored_buffer.lock().unwrap();
        *lock = Some(buffer.clone());
        Ok(())
    }

    fn clear(&self) -> Result<(), ClipboardDriverError> {
        let mut lock = self.stored_buffer.lock().unwrap();
        *lock = None;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_mock_clipboard_lifecycle() {
        let driver = MockClipboardDriver::new();
        assert!(driver.get_last_copied().is_none());

        let buffer = RawDisplayBuffer::new(
            10,
            10,
            40,
            crate::domain::buffer::PixelFormat::Rgba8,
            vec![255; 400],
        )
        .unwrap();

        driver.copy_image(&buffer).unwrap();
        let copied = driver.get_last_copied().unwrap();
        assert_eq!(copied.data.len(), 400);

        driver.clear().unwrap();
        assert!(driver.get_last_copied().is_none());
    }
}
