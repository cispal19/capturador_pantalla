pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod ports;
pub mod presentation;

use presentation::commands::{
    cancel_capture_session, commit_selection_region, get_app_info, get_connected_monitors,
    get_session_state, normalize_bounding_box, register_capture_shortcut, trigger_screen_capture,
};
use presentation::tray::create_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            #[cfg(desktop)]
            {
                let _tray = create_tray(app.handle())?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            normalize_bounding_box,
            get_session_state,
            get_connected_monitors,
            trigger_screen_capture,
            register_capture_shortcut,
            cancel_capture_session,
            commit_selection_region
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::presentation::commands::get_app_info;

    #[test]
    fn test_app_lifecycle_metadata() {
        let info = get_app_info();
        assert_eq!(info.name, "capturador-pantalla");
        assert_eq!(info.version, "1.0.0");
        assert!(info.local_first_verified);
    }
}
