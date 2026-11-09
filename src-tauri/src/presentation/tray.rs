use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Runtime,
};

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<TrayIcon<R>, tauri::Error> {
    let capture_i = MenuItem::with_id(
        app,
        "capture",
        "Tomar Captura (Cmd+Shift+X)",
        true,
        None::<&str>,
    )?;
    let preferences_i =
        MenuItem::with_id(app, "preferences", "Preferencias...", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Salir", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&capture_i, &preferences_i, &quit_i])?;

    TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "capture" => {
                // Handled via IPC / internal event
            }
            "preferences" => {
                // Handled via preferences window
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)
}
