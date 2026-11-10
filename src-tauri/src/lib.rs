pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod ports;
pub mod presentation;

use presentation::commands::{
    apply_obfuscation, cancel_capture_session, cancel_screen_recording, commit_selection_region,
    copy_capture_to_clipboard, export_frames_to_gif, finish_capture_session_flow, get_app_info,
    get_connected_monitors, get_preferences, get_recording_status, get_session_state,
    hide_capture_overlay, normalize_bounding_box, pause_screen_recording,
    register_capture_shortcut, reset_preferences, resume_screen_recording, save_capture_to_file,
    start_capture_session_flow, start_screen_recording, stop_screen_recording, sync_annotations,
    trigger_screen_capture, update_preferences,
};
use presentation::tray::create_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            #[cfg(desktop)]
            {
                let _tray = create_tray(app.handle())?;

                use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
                let app_handle = app.handle().clone();

                let shortcuts_to_register = [
                    "Cmd+Shift+X",
                    "Command+Shift+X",
                    "Ctrl+Shift+X",
                    "CommandOrControl+Shift+X",
                ];

                for sc_str in shortcuts_to_register {
                    if let Ok(shortcut) = sc_str.parse::<Shortcut>() {
                        let handle = app_handle.clone();
                        let _ = app
                            .global_shortcut()
                            .on_shortcut(shortcut, move |_app, _sc, event| {
                                if event.state() == ShortcutState::Pressed {
                                    let h = handle.clone();
                                    std::thread::spawn(move || {
                                        let _ =
                                            crate::presentation::commands::start_capture_session_flow(
                                                h, None,
                                            );
                                    });
                                }
                            });
                    }
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
            hide_capture_overlay,
            finish_capture_session_flow
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        #[cfg(target_os = "macos")]
        if let tauri::RunEvent::Reopen { .. } = event {
            use tauri::Manager;
            if let Some(main_win) = app_handle.get_webview_window("main") {
                let _ = main_win.show();
                let _ = main_win.unminimize();
                let _ = main_win.set_focus();
            }
        }
    });
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

    #[test]
    fn test_shortcut_parsing() {
        use tauri_plugin_global_shortcut::Shortcut;
        let sc_cmd_or_ctrl = "CommandOrControl+Shift+X".parse::<Shortcut>();
        println!("CommandOrControl result: {:?}", sc_cmd_or_ctrl.is_ok());

        let sc_mac = "Command+Shift+X".parse::<Shortcut>();
        assert!(sc_mac.is_ok(), "Command+Shift+X should parse");

        let sc_cmd = "Cmd+Shift+X".parse::<Shortcut>();
        assert!(sc_cmd.is_ok(), "Cmd+Shift+X should parse");

        let sc_ctrl = "Ctrl+Shift+X".parse::<Shortcut>();
        assert!(sc_ctrl.is_ok(), "Ctrl+Shift+X should parse");
    }
}
