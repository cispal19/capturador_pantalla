pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod ports;
pub mod presentation;

use presentation::commands::{
    apply_obfuscation, cancel_capture_session, cancel_screen_recording, commit_selection_region,
    copy_capture_to_clipboard, export_frames_to_gif, get_app_info, get_connected_monitors,
    get_preferences, get_recording_status, get_session_state, hide_capture_overlay,
    normalize_bounding_box, pause_screen_recording, register_capture_shortcut, reset_preferences,
    resume_screen_recording, save_capture_to_file, start_capture_session_flow,
    start_screen_recording, stop_screen_recording, sync_annotations, trigger_screen_capture,
    update_preferences,
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

                use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
                let app_handle = app.handle().clone();
                if let Ok(shortcut) = "CommandOrControl+Shift+X".parse::<Shortcut>() {
                    let _ = app
                        .global_shortcut()
                        .on_shortcut(shortcut, move |_app, _sc, event| {
                            if event.state() == ShortcutState::Pressed {
                                let handle = app_handle.clone();
                                std::thread::spawn(move || {
                                    let _ =
                                        crate::presentation::commands::start_capture_session_flow(
                                            handle, None,
                                        );
                                });
                            }
                        });
                }
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
            commit_selection_region,
            apply_obfuscation,
            sync_annotations,
            copy_capture_to_clipboard,
            save_capture_to_file,
            get_preferences,
            update_preferences,
            reset_preferences,
            start_screen_recording,
            pause_screen_recording,
            resume_screen_recording,
            stop_screen_recording,
            cancel_screen_recording,
            get_recording_status,
            export_frames_to_gif,
            start_capture_session_flow,
            hide_capture_overlay
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
