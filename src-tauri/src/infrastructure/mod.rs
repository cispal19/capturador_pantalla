pub mod mock_capture;
pub mod mock_shortcut;
pub mod permissions;
pub mod tauri_shortcut;
pub mod xcap_capture;

pub use mock_capture::MockScreenCaptureDriver;
pub use mock_shortcut::MockShortcutDriver;
pub use permissions::SystemPermissionsService;
pub use tauri_shortcut::TauriGlobalShortcutDriver;
pub use xcap_capture::XcapScreenCaptureDriver;
