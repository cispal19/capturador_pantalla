use crate::domain::{
    buffer::{PixelFormat, RawDisplayBuffer},
    display::DisplayInfo,
};
use crate::ports::{CaptureDriverError, IScreenCaptureDriver};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MockScreenCaptureDriver {
    displays: Arc<Mutex<Vec<DisplayInfo>>>,
}

impl Default for MockScreenCaptureDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl MockScreenCaptureDriver {
    pub fn new() -> Self {
        let default_displays = vec![
            DisplayInfo::new(
                0,
                "Built-in Retina Display".to_string(),
                1728,
                1117,
                3456,
                2234,
                2.0,
                true,
            )
            .unwrap(),
            DisplayInfo::new(
                1,
                "External 4K Monitor".to_string(),
                1920,
                1080,
                3840,
                2160,
                2.0,
                false,
            )
            .unwrap(),
        ];

        Self {
            displays: Arc::new(Mutex::new(default_displays)),
        }
    }

    pub fn with_displays(displays: Vec<DisplayInfo>) -> Self {
        Self {
            displays: Arc::new(Mutex::new(displays)),
        }
    }

    fn generate_gradient_buffer(
        width: u32,
        height: u32,
    ) -> Result<RawDisplayBuffer, CaptureDriverError> {
        let stride = width as usize * 4;
        let mut data = vec![0u8; stride * height as usize];

        // Fill with deterministic gradient pattern for testing
        for y in 0..height {
            for x in 0..width {
                let idx = (y as usize * stride) + (x as usize * 4);
                data[idx] = ((x * 255) / width.max(1)) as u8; // R
                data[idx + 1] = ((y * 255) / height.max(1)) as u8; // G
                data[idx + 2] = 128; // B
                data[idx + 3] = 255; // A
            }
        }

        RawDisplayBuffer::new(width, height, stride, PixelFormat::Rgba8, data)
            .map_err(|e| CaptureDriverError::Internal(e.to_string()))
    }
}

impl IScreenCaptureDriver for MockScreenCaptureDriver {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, CaptureDriverError> {
        let displays = self.displays.lock().unwrap();
        if displays.is_empty() {
            return Err(CaptureDriverError::NoDisplayFound);
        }
        Ok(displays.clone())
    }

    fn capture_active_display(&self) -> Result<RawDisplayBuffer, CaptureDriverError> {
        let displays = self.displays.lock().unwrap();
        let primary = displays
            .iter()
            .find(|d| d.is_primary)
            .or_else(|| displays.first())
            .ok_or(CaptureDriverError::NoDisplayFound)?;

        Self::generate_gradient_buffer(primary.physical_width, primary.physical_height)
    }

    fn capture_display_by_id(
        &self,
        display_id: u32,
    ) -> Result<RawDisplayBuffer, CaptureDriverError> {
        let displays = self.displays.lock().unwrap();
        let display = displays
            .iter()
            .find(|d| d.id == display_id)
            .ok_or(CaptureDriverError::DisplayNotFound(display_id))?;

        Self::generate_gradient_buffer(display.physical_width, display.physical_height)
    }

    fn capture_all_displays(
        &self,
    ) -> Result<Vec<(DisplayInfo, RawDisplayBuffer)>, CaptureDriverError> {
        let displays = self.displays.lock().unwrap();
        if displays.is_empty() {
            return Err(CaptureDriverError::NoDisplayFound);
        }

        let mut results = Vec::new();
        for d in displays.iter() {
            let buffer = Self::generate_gradient_buffer(d.physical_width, d.physical_height)?;
            results.push((d.clone(), buffer));
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_screen_capture_gradient() {
        let driver = MockScreenCaptureDriver::new();
        let buffer = driver.capture_active_display().unwrap();

        assert_eq!(buffer.width, 3456);
        assert_eq!(buffer.height, 2234);
        assert_eq!(buffer.data.len(), 3456 * 2234 * 4);
        assert_eq!(buffer.format, PixelFormat::Rgba8);
    }

    #[test]
    fn test_mock_multi_monitor_enumeration() {
        let driver = MockScreenCaptureDriver::new();
        let displays = driver.get_displays().unwrap();
        assert_eq!(displays.len(), 2);
        assert_eq!(displays[0].name, "Built-in Retina Display");
        assert_eq!(displays[1].name, "External 4K Monitor");
    }

    #[test]
    fn test_mock_capture_by_id() {
        let driver = MockScreenCaptureDriver::new();
        let buf1 = driver.capture_display_by_id(1).unwrap();
        assert_eq!(buf1.width, 3840);
        assert_eq!(buf1.height, 2160);

        let err = driver.capture_display_by_id(999);
        assert!(matches!(err, Err(CaptureDriverError::DisplayNotFound(999))));
    }
}
