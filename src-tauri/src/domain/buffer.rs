use super::error::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PixelFormat {
    Rgba8,
    Bgra8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureSessionState {
    Idle,
    Selecting,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawDisplayBuffer {
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub format: PixelFormat,
    pub data: Vec<u8>,
}

impl RawDisplayBuffer {
    pub fn new(
        width: u32,
        height: u32,
        stride: usize,
        format: PixelFormat,
        data: Vec<u8>,
    ) -> Result<Self, DomainError> {
        let expected = stride * height as usize;
        if data.len() != expected {
            return Err(DomainError::BufferLengthMismatch {
                expected,
                actual: data.len(),
            });
        }
        Ok(Self {
            width,
            height,
            stride,
            format,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_buffer() {
        let width = 2;
        let height = 2;
        let stride = 8; // 2 pixels * 4 bytes
        let data = vec![255; stride * height as usize];

        let buffer =
            RawDisplayBuffer::new(width, height, stride, PixelFormat::Rgba8, data).unwrap();
        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 2);
        assert_eq!(buffer.data.len(), 16);
    }

    #[test]
    fn test_buffer_length_mismatch() {
        let width = 2;
        let height = 2;
        let stride = 8;
        let invalid_data = vec![0; 10]; // should be 16

        assert!(
            RawDisplayBuffer::new(width, height, stride, PixelFormat::Rgba8, invalid_data).is_err()
        );
    }
}
