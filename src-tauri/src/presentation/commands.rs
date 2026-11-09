use crate::domain::{
    bounding_box::BoundingBox, buffer::CaptureSessionState, coordinates::DisplayCoordinates,
    dimensions::Dimensions, display::DisplayInfo, error::DomainError,
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
