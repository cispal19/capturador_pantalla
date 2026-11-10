use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Emitter, Manager,
};

pub fn create_tray(app: &AppHandle) -> Result<TrayIcon, tauri::Error> {
    let capture_i = MenuItem::with_id(
        app,
        "capture",
        "Tomar Captura (Cmd+Shift+X)",
        true,
        None::<&str>,
    )?;
    let toggle_i = MenuItem::with_id(
        app,
        "toggle_window",
        "Mostrar / Ocultar Ventana",
        true,
        None::<&str>,
    )?;
    let preferences_i =
        MenuItem::with_id(app, "preferences", "Preferencias...", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Salir", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&capture_i, &toggle_i, &preferences_i, &quit_i])?;

    let icon_bytes = include_bytes!("../../icons/tray.png");
    let icon = if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Some(tauri::image::Image::new_owned(rgba.into_raw(), width, height))
    } else {
        app.default_window_icon().cloned()
    };

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .tooltip("ianShort");

    if let Some(tray_icon) = icon {
        builder = builder.icon(tray_icon).icon_as_template(true);
    }

    builder
        .on_menu_event(|app, event| match event.id.as_ref() {
            "capture" => {
                let app_handle = app.clone();
                std::thread::spawn(move || {
                    let _ =
                        crate::presentation::commands::start_capture_session_flow(app_handle, None);
                });
            }
            "toggle_window" => {
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(true) = window.is_visible() {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                }
            }
            "preferences" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
                let _ = app.emit("open-preferences", ());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)
}
