use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Emitter, Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<TrayIcon<R>, tauri::Error> {
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

    TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "capture" => {
                let _ = app.emit("trigger-capture", ());
            }
            "toggle_window" => {
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(true) = window.is_visible() {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
            "preferences" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
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
