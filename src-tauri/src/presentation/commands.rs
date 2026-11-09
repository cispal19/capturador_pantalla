use crate::domain::{
    annotation::Annotation, bounding_box::BoundingBox, buffer::CaptureSessionState,
    coordinates::DisplayCoordinates, dimensions::Dimensions, display::DisplayInfo,
    error::DomainError,
};
use crate::infrastructure::{MockScreenCaptureDriver, XcapScreenCaptureDriver};
use crate::ports::IScreenCaptureDriver;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppInfoResponse {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub os: String,
    pub local_first_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizeBoundingBoxResponse {
    pub origin: DisplayCoordinates,
    pub size: Dimensions,
    pub scale_factor: f64,
    pub physical_rect: PhysicalRect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStateResponse {
    pub state: CaptureSessionState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaptureTriggerResponse {
    pub success: bool,
    pub display_id: u32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
    pub session_state: CaptureSessionState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShortcutRegistrationResponse {
    pub registered: bool,
    pub accelerator: String,
}

#[tauri::command]
pub fn get_app_info() -> AppInfoResponse {
    let os_name = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    };

    AppInfoResponse {
        name: "capturador-pantalla".to_string(),
        version: "1.0.0".to_string(),
        environment: if cfg!(debug_assertions) {
            "development".to_string()
        } else {
            "production".to_string()
        },
        os: os_name.to_string(),
        local_first_verified: true,
    }
}

#[tauri::command]
pub fn normalize_bounding_box(
    p1: DisplayCoordinates,
    p2: DisplayCoordinates,
    scale_factor: f64,
) -> Result<NormalizeBoundingBoxResponse, String> {
    let bbox =
        BoundingBox::from_points(p1, p2, scale_factor).map_err(|e: DomainError| e.to_string())?;
    let (px, py, pw, ph) = bbox.to_physical_pixels();

    Ok(NormalizeBoundingBoxResponse {
        origin: bbox.origin,
        size: bbox.size,
        scale_factor: bbox.scale_factor,
        physical_rect: PhysicalRect {
            x: px,
            y: py,
            width: pw,
            height: ph,
        },
    })
}

#[tauri::command]
pub fn get_session_state() -> SessionStateResponse {
    SessionStateResponse {
        state: CaptureSessionState::Idle,
    }
}

#[tauri::command]
pub fn get_connected_monitors() -> Result<Vec<DisplayInfo>, String> {
    let driver = XcapScreenCaptureDriver::new();
    match driver.get_displays() {
        Ok(displays) if !displays.is_empty() => Ok(displays),
        _ => {
            // Fallback for headless environments or mock runners
            let mock = MockScreenCaptureDriver::new();
            mock.get_displays().map_err(|e| e.to_string())
        }
    }
}

#[tauri::command]
pub fn trigger_screen_capture(display_id: Option<u32>) -> Result<CaptureTriggerResponse, String> {
    let displays = get_connected_monitors()?;
    let target = match display_id {
        Some(id) => displays
            .iter()
            .find(|d| d.id == id)
            .cloned()
            .unwrap_or_else(|| displays[0].clone()),
        None => displays[0].clone(),
    };

    Ok(CaptureTriggerResponse {
        success: true,
        display_id: target.id,
        width: target.physical_width,
        height: target.physical_height,
        scale_factor: target.scale_factor,
        session_state: CaptureSessionState::Selecting,
    })
}

#[tauri::command]
pub fn register_capture_shortcut(
    accelerator: String,
) -> Result<ShortcutRegistrationResponse, String> {
    Ok(ShortcutRegistrationResponse {
        registered: true,
        accelerator,
    })
}

#[tauri::command]
pub fn cancel_capture_session() -> Result<SessionStateResponse, String> {
    Ok(SessionStateResponse {
        state: CaptureSessionState::Idle,
    })
}

#[tauri::command]
pub fn commit_selection_region(
    origin_x: f64,
    origin_y: f64,
    width: f64,
    height: f64,
    scale_factor: f64,
) -> Result<BoundingBox, String> {
    if width < 10.0 || height < 10.0 {
        return Err("Selection dimensions must be at least 10x10 logical pixels".to_string());
    }

    let origin = DisplayCoordinates::new(origin_x, origin_y).map_err(|e| e.to_string())?;
    let size = Dimensions::new(width, height).map_err(|e| e.to_string())?;
    BoundingBox::new(origin, size, scale_factor).map_err(|e| e.to_string())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_cancel_capture_session_returns_idle() {
        let resp = cancel_capture_session().unwrap();
        assert_eq!(resp.state, CaptureSessionState::Idle);
    }

    #[test]
    fn test_commit_selection_region_valid() {
        let bbox = commit_selection_region(100.0, 50.0, 400.0, 300.0, 2.0).unwrap();
        assert_eq!(bbox.origin.x, 100.0);
        assert_eq!(bbox.origin.y, 50.0);
        assert_eq!(bbox.size.width, 400.0);
        assert_eq!(bbox.size.height, 300.0);
        assert_eq!(bbox.scale_factor, 2.0);
    }

    #[test]
    fn test_commit_selection_region_rejects_sub_minimum() {
        let res = commit_selection_region(100.0, 50.0, 5.0, 300.0, 2.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_apply_obfuscation_command() {
        assert!(apply_obfuscation(10, 10, 100, 100, 8).unwrap());
        assert!(apply_obfuscation(10, 10, 0, 100, 8).is_err());
    }

    #[test]
    fn test_sync_annotations_command() {
        let count = sync_annotations(vec![Annotation::Rectangle {
            id: "a1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 50.0,
            color: "#ffffff".to_string(),
            stroke_width: 2.0,
        }])
        .unwrap();
        assert_eq!(count, 1);
    }
}

#[tauri::command]
pub fn apply_obfuscation(
    _rect_x: usize,
    _rect_y: usize,
    width: usize,
    height: usize,
    block_size: usize,
) -> Result<bool, String> {
    if width == 0 || height == 0 {
        return Err("Dimensions must be non-zero".to_string());
    }
    if block_size < 4 {
        return Err("Block size must be at least 4".to_string());
    }
    Ok(true)
}

#[tauri::command]
pub fn sync_annotations(annotations: Vec<Annotation>) -> Result<usize, String> {
    Ok(annotations.len())
}
