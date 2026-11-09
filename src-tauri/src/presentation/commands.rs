use crate::domain::{
    bounding_box::BoundingBox, buffer::CaptureSessionState, coordinates::DisplayCoordinates,
    dimensions::Dimensions, error::DomainError,
};
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
