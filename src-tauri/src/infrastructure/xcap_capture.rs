use crate::domain::{
    buffer::{PixelFormat, RawDisplayBuffer},
    display::DisplayInfo,
};
use crate::ports::{CaptureDriverError, IScreenCaptureDriver};

pub struct XcapScreenCaptureDriver;

impl Default for XcapScreenCaptureDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl XcapScreenCaptureDriver {
    pub fn new() -> Self {
        Self
    }
}

impl IScreenCaptureDriver for XcapScreenCaptureDriver {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, CaptureDriverError> {
        let monitors =
            xcap::Monitor::all().map_err(|e| CaptureDriverError::Internal(e.to_string()))?;

        if monitors.is_empty() {
            return Err(CaptureDriverError::NoDisplayFound);
        }

        let mut displays = Vec::new();
        for (idx, m) in monitors.iter().enumerate() {
            let scale = m.scale_factor().unwrap_or(1.0) as f64;
            let p_width = m.width().unwrap_or(1920);
            let p_height = m.height().unwrap_or(1080);
            let l_width = if scale > 0.0 {
                (p_width as f64 / scale).round() as u32
            } else {
                p_width
            };
            let l_height = if scale > 0.0 {
                (p_height as f64 / scale).round() as u32
            } else {
                p_height
            };
            let is_primary = m.is_primary().unwrap_or(false);
            let name = m.name().unwrap_or_else(|_| format!("Display {}", idx));

            let info = DisplayInfo::new(
                idx as u32,
                name,
                l_width.max(1),
                l_height.max(1),
                p_width.max(1),
                p_height.max(1),
                if scale > 0.0 { scale } else { 1.0 },
                is_primary,
            )
            .map_err(|e| CaptureDriverError::Internal(e.to_string()))?;

            displays.push(info);
        }

        Ok(displays)
    }

    fn capture_active_display(&self) -> Result<RawDisplayBuffer, CaptureDriverError> {
        let monitors =
            xcap::Monitor::all().map_err(|e| CaptureDriverError::Internal(e.to_string()))?;

        let primary = monitors
            .iter()
            .find(|m| m.is_primary().unwrap_or(false))
            .or_else(|| monitors.first())
            .ok_or(CaptureDriverError::NoDisplayFound)?;

        let image = primary
            .capture_image()
            .map_err(|e| CaptureDriverError::Internal(e.to_string()))?;

        let width = image.width();
        let height = image.height();
        let stride = width as usize * 4;
        let data = image.into_raw();

        RawDisplayBuffer::new(width, height, stride, PixelFormat::Rgba8, data)
            .map_err(|e| CaptureDriverError::Internal(e.to_string()))
    }

    fn capture_display_by_id(
        &self,
        display_id: u32,
    ) -> Result<RawDisplayBuffer, CaptureDriverError> {
        let monitors =
            xcap::Monitor::all().map_err(|e| CaptureDriverError::Internal(e.to_string()))?;

        let monitor = monitors
            .get(display_id as usize)
            .ok_or(CaptureDriverError::DisplayNotFound(display_id))?;

        let image = monitor
            .capture_image()
            .map_err(|e| CaptureDriverError::Internal(e.to_string()))?;

        let width = image.width();
        let height = image.height();
        let stride = width as usize * 4;
        let data = image.into_raw();

        RawDisplayBuffer::new(width, height, stride, PixelFormat::Rgba8, data)
            .map_err(|e| CaptureDriverError::Internal(e.to_string()))
    }

    fn capture_all_displays(
        &self,
    ) -> Result<Vec<(DisplayInfo, RawDisplayBuffer)>, CaptureDriverError> {
        let displays = self.get_displays()?;
        let mut results = Vec::new();

        for d in displays {
            let buffer = self.capture_display_by_id(d.id)?;
            results.push((d, buffer));
        }

        Ok(results)
    }
}
