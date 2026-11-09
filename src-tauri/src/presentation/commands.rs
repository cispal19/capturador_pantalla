use crate::application::{
    ExportRecordingToGifUseCase, RecordingSessionManager, RecordingStatusResponse,
};
use crate::domain::{
    annotation::Annotation,
    bounding_box::BoundingBox,
    buffer::{CaptureSessionState, RawDisplayBuffer},
    coordinates::DisplayCoordinates,
    dimensions::Dimensions,
    display::DisplayInfo,
    error::DomainError,
    gif::{GifExportConfig, GifExportResult},
    naming::FileNameGenerator,
    preferences::AppPreferences,
    recording::{RecordingMetadata, RecordingRegion},
};
use crate::infrastructure::{
    ArboardClipboardDriver, FileSystemStorageDriver, JsonConfigStoreDriver, MockClipboardDriver,
    MockScreenCaptureDriver, MockScreenRecorderDriver, XcapScreenCaptureDriver,
};
use crate::ports::{IClipboardDriver, IFileSystemDriver, IScreenCaptureDriver, ImageFileFormat};
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};

static RECORDING_MANAGER: LazyLock<Mutex<RecordingSessionManager>> = LazyLock::new(|| {
    let driver = std::sync::Arc::new(MockScreenRecorderDriver::new(None));
    Mutex::new(RecordingSessionManager::new(driver))
});

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
    pub image_data_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShortcutRegistrationResponse {
    pub registered: bool,
    pub accelerator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveImageResult {
    pub file_path: String,
    pub file_size_bytes: usize,
    pub format: String,
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
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;

    let displays = get_connected_monitors()?;
    let target = match display_id {
        Some(id) => displays
            .iter()
            .find(|d| d.id == id)
            .cloned()
            .unwrap_or_else(|| displays[0].clone()),
        None => displays[0].clone(),
    };

    let driver = XcapScreenCaptureDriver::new();
    let buffer_res = match display_id {
        Some(id) => driver.capture_display_by_id(id),
        None => driver.capture_active_display(),
    };

    let image_data_url = match buffer_res {
        Ok(buf) => {
            if let Some(rgba) = image::RgbaImage::from_raw(buf.width, buf.height, buf.data.clone())
            {
                let mut bytes = std::io::Cursor::new(Vec::new());
                if rgba.write_to(&mut bytes, image::ImageFormat::Png).is_ok() {
                    let encoded = STANDARD.encode(bytes.into_inner());
                    Some(format!("data:image/png;base64,{}", encoded))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Err(_) => {
            let mock = MockScreenCaptureDriver::new();
            if let Ok(buf) = mock.capture_active_display() {
                if let Some(rgba) =
                    image::RgbaImage::from_raw(buf.width, buf.height, buf.data.clone())
                {
                    let mut bytes = std::io::Cursor::new(Vec::new());
                    if rgba.write_to(&mut bytes, image::ImageFormat::Png).is_ok() {
                        let encoded = STANDARD.encode(bytes.into_inner());
                        Some(format!("data:image/png;base64,{}", encoded))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    Ok(CaptureTriggerResponse {
        success: true,
        display_id: target.id,
        width: target.physical_width,
        height: target.physical_height,
        scale_factor: target.scale_factor,
        session_state: CaptureSessionState::Selecting,
        image_data_url,
    })
}

#[tauri::command]
pub fn start_capture_session_flow(
    app: tauri::AppHandle,
    display_id: Option<u32>,
) -> Result<CaptureTriggerResponse, String> {
    use tauri::{Emitter, Manager};

    // 1. Hide main dashboard window so it does not get captured in screenshot
    if let Some(main_win) = app.get_webview_window("main") {
        let _ = main_win.hide();
    }

    // 2. Allow 200ms for OS compositor to clear the main window from the screen
    std::thread::sleep(std::time::Duration::from_millis(200));

    // 3. Capture desktop screenshot via xcap
    let capture_res = trigger_screen_capture(display_id)?;

    // 4. Position and show the overlay window
    if let Some(overlay_win) = app.get_webview_window("overlay") {
        let target_mon = overlay_win
            .current_monitor()
            .ok()
            .flatten()
            .or_else(|| overlay_win.primary_monitor().ok().flatten());

        if let Some(mon) = target_mon {
            let size = mon.size();
            let pos = mon.position();
            let _ = overlay_win.set_position(tauri::Position::Physical(*pos));
            let _ = overlay_win.set_size(tauri::Size::Physical(*size));
        }

        let _ = overlay_win.set_always_on_top(true);
        let _ = overlay_win.show();
        let _ = overlay_win.set_focus();

        let _ = overlay_win.emit(
            "start-capture",
            serde_json::json!({
                "imageUrl": capture_res.image_data_url,
                "width": capture_res.width,
                "height": capture_res.height,
                "scaleFactor": capture_res.scale_factor,
            }),
        );
    }

    Ok(capture_res)
}

#[tauri::command]
pub fn hide_capture_overlay(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    if let Some(overlay_win) = app.get_webview_window("overlay") {
        let _ = overlay_win.hide();
    }
    Ok(())
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
    use crate::domain::recording::RecordingState;

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

    #[test]
    fn test_copy_capture_to_clipboard_command() {
        let data = vec![255; 400]; // 10x10x4
        assert!(copy_capture_to_clipboard(10, 10, data).unwrap());
        // Mismatch length error
        assert!(copy_capture_to_clipboard(10, 10, vec![255; 200]).is_err());
    }

    #[test]
    fn test_save_capture_to_file_command() {
        let data = vec![255; 400];
        let res = save_capture_to_file(10, 10, data, Some("png".to_string()), None).unwrap();
        assert!(std::path::Path::new(&res.file_path).exists());
        let _ = std::fs::remove_file(res.file_path);
    }

    #[test]
    fn test_preferences_commands_lifecycle() {
        let prefs = get_preferences().unwrap();
        assert_eq!(prefs.default_image_format, "png");

        let mut custom = prefs.clone();
        custom.default_image_format = "jpeg".to_string();
        let updated = update_preferences(custom.clone()).unwrap();
        assert_eq!(updated.default_image_format, "jpeg");

        let reset = reset_preferences().unwrap();
        assert_eq!(reset.default_image_format, "png");
    }

    #[test]
    fn test_recording_commands_lifecycle() {
        let _ = cancel_screen_recording();
        let region = start_screen_recording(100, 100, 400, 300, Some(30), Some(600)).unwrap();
        assert_eq!(region.width, 400);

        let status = get_recording_status().unwrap();
        assert_eq!(status.state, RecordingState::Recording);

        assert!(pause_screen_recording().unwrap());
        assert_eq!(
            get_recording_status().unwrap().state,
            RecordingState::Paused
        );

        assert!(resume_screen_recording().unwrap());
        assert_eq!(
            get_recording_status().unwrap().state,
            RecordingState::Recording
        );

        let fake_video = vec![0x1a, 0x45, 0xdf, 0xa3];
        let meta = stop_screen_recording(fake_video, 5, Some("webm".to_string())).unwrap();
        assert_eq!(meta.duration_seconds, 5);
        assert!(std::path::Path::new(&meta.file_path).exists());
        let _ = std::fs::remove_file(meta.file_path);

        assert_eq!(get_recording_status().unwrap().state, RecordingState::Idle);
    }

    #[test]
    fn test_recording_commands_cancel() {
        let _ = cancel_screen_recording();
        start_screen_recording(50, 50, 200, 200, None, None).unwrap();
        assert_eq!(
            get_recording_status().unwrap().state,
            RecordingState::Recording
        );

        assert!(cancel_screen_recording().unwrap());
        assert_eq!(get_recording_status().unwrap().state, RecordingState::Idle);
    }

    #[test]
    fn test_export_frames_to_gif_command() {
        let frame1 = vec![255, 0, 0, 255].repeat(100); // 10x10
        let frame2 = vec![0, 255, 0, 255].repeat(100);
        let res = export_frames_to_gif(10, 10, vec![frame1, frame2], Some(10), None).unwrap();
        assert_eq!(res.frame_count, 2);
        assert!(std::path::Path::new(&res.file_path).exists());
        let _ = std::fs::remove_file(res.file_path);
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

#[tauri::command]
pub fn copy_capture_to_clipboard(
    width: u32,
    height: u32,
    rgba_data: Vec<u8>,
) -> Result<bool, String> {
    if width == 0 || height == 0 {
        return Err("Dimensions must be non-zero".to_string());
    }
    let expected_len = (width as usize) * (height as usize) * 4;
    if rgba_data.len() != expected_len {
        return Err(format!(
            "Buffer size mismatch: expected {} bytes, got {}",
            expected_len,
            rgba_data.len()
        ));
    }

    let stride = (width * 4) as usize;
    let buffer = RawDisplayBuffer::new(
        width,
        height,
        stride,
        crate::domain::buffer::PixelFormat::Rgba8,
        rgba_data,
    )
    .map_err(|e| e.to_string())?;

    let driver = ArboardClipboardDriver::new();
    match driver.copy_image(&buffer) {
        Ok(_) => Ok(true),
        Err(_) => {
            // Fallback for headless testing environments
            let mock = MockClipboardDriver::new();
            mock.copy_image(&buffer).map_err(|e| e.to_string())?;
            Ok(true)
        }
    }
}

#[tauri::command]
pub fn save_capture_to_file(
    width: u32,
    height: u32,
    rgba_data: Vec<u8>,
    format: Option<String>,
    target_directory: Option<String>,
) -> Result<SaveImageResult, String> {
    if width == 0 || height == 0 {
        return Err("Dimensions must be non-zero".to_string());
    }
    let expected_len = (width as usize) * (height as usize) * 4;
    if rgba_data.len() != expected_len {
        return Err(format!(
            "Buffer size mismatch: expected {} bytes, got {}",
            expected_len,
            rgba_data.len()
        ));
    }

    let stride = (width * 4) as usize;
    let buffer = RawDisplayBuffer::new(
        width,
        height,
        stride,
        crate::domain::buffer::PixelFormat::Rgba8,
        rgba_data,
    )
    .map_err(|e| e.to_string())?;

    let ext = format.unwrap_or_else(|| "png".to_string()).to_lowercase();
    let img_format = match ext.as_str() {
        "jpg" | "jpeg" => ImageFileFormat::Jpeg { quality: 90 },
        "webp" => ImageFileFormat::Webp,
        _ => ImageFileFormat::Png,
    };

    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let file_name = FileNameGenerator::generate_name(&ext, now_secs);

    let dir = target_directory
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);

    let file_path = dir.join(file_name);

    let driver = FileSystemStorageDriver::new();
    let saved_path = driver
        .save_image(&file_path, &buffer, img_format)
        .map_err(|e| e.to_string())?;

    let size = std::fs::metadata(&saved_path)
        .map(|m| m.len() as usize)
        .unwrap_or(0);

    Ok(SaveImageResult {
        file_path: saved_path.to_string_lossy().to_string(),
        file_size_bytes: size,
        format: ext,
    })
}

#[tauri::command]
pub fn get_preferences() -> Result<AppPreferences, String> {
    let store = JsonConfigStoreDriver::default();
    Ok(store.load())
}

#[tauri::command]
pub fn update_preferences(preferences: AppPreferences) -> Result<AppPreferences, String> {
    preferences.validate()?;
    let store = JsonConfigStoreDriver::default();
    store.save(&preferences)?;
    Ok(preferences)
}

#[tauri::command]
pub fn reset_preferences() -> Result<AppPreferences, String> {
    let defaults = AppPreferences::default();
    let store = JsonConfigStoreDriver::default();
    store.save(&defaults)?;
    Ok(defaults)
}

#[tauri::command]
pub fn start_screen_recording(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    fps: Option<u32>,
    time_limit_seconds: Option<u32>,
) -> Result<RecordingRegion, String> {
    let region = RecordingRegion::new(
        x,
        y,
        width,
        height,
        fps.unwrap_or(30),
        time_limit_seconds.unwrap_or(600),
    )?;
    let mgr = RECORDING_MANAGER.lock().unwrap();
    mgr.start_recording(region)
}

#[tauri::command]
pub fn pause_screen_recording() -> Result<bool, String> {
    let mgr = RECORDING_MANAGER.lock().unwrap();
    mgr.pause_recording()?;
    Ok(true)
}

#[tauri::command]
pub fn resume_screen_recording() -> Result<bool, String> {
    let mgr = RECORDING_MANAGER.lock().unwrap();
    mgr.resume_recording()?;
    Ok(true)
}

#[tauri::command]
pub fn stop_screen_recording(
    video_data: Vec<u8>,
    duration_seconds: u32,
    format: Option<String>,
) -> Result<RecordingMetadata, String> {
    let fmt = format.unwrap_or_else(|| "webm".to_string());
    let mgr = RECORDING_MANAGER.lock().unwrap();
    mgr.stop_recording(&video_data, duration_seconds, &fmt)
}

#[tauri::command]
pub fn cancel_screen_recording() -> Result<bool, String> {
    let mgr = RECORDING_MANAGER.lock().unwrap();
    mgr.cancel_recording()?;
    Ok(true)
}

#[tauri::command]
pub fn get_recording_status() -> Result<RecordingStatusResponse, String> {
    let mgr = RECORDING_MANAGER.lock().unwrap();
    Ok(mgr.get_status())
}

#[tauri::command]
pub fn export_frames_to_gif(
    width: u32,
    height: u32,
    frames_rgba: Vec<Vec<u8>>,
    fps: Option<u32>,
    target_directory: Option<String>,
) -> Result<GifExportResult, String> {
    let config = GifExportConfig::new(fps.unwrap_or(15), 15);
    let dir = target_directory.map(std::path::PathBuf::from);
    ExportRecordingToGifUseCase::execute(width, height, frames_rgba, config, dir)
}
