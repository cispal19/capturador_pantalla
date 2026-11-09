pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod ports;
pub mod presentation;

use presentation::commands::{get_app_info, get_session_state, normalize_bounding_box};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            normalize_bounding_box,
            get_session_state
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
