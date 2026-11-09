# Research: Settings and User Preferences (Feature 006)

## 1. Technical Context & Cross-Platform Configuration Storage

### Decision: User-Space JSON File
- **Linux / macOS**:
  - Primary path: `$XDG_CONFIG_HOME/capturador-pantalla/config.json` (if set) or `$HOME/.config/capturador-pantalla/config.json`.
  - macOS standard fallback: `$HOME/Library/Application Support/capturador-pantalla/config.json` or `$HOME/.config/capturador-pantalla/config.json`. Using `~/.config/capturador-pantalla/config.json` aligns with corporate CLI/tools conventions and TRD spec.
- **Windows**:
  - Primary path: `%APPDATA%\capturador-pantalla\config.json`.
- **In-Memory Fallback**:
  - If filesystem access fails, default preferences remain in memory so capture and annotation workflows never crash.

### Serialization & Schema Evolution
- Using `serde` and `serde_json`.
- All fields in `AppPreferences` must have default values using `#[serde(default)]` to guarantee backward and forward compatibility:
  - If a user upgrades and the JSON file lacks new fields, `serde` deserializes defaults without failing.
  - If `config.json` has invalid syntax or is corrupted, the application logs a warning, loads safe defaults, and saves a repaired valid JSON file.

---

## 2. Dynamic Shortcut Re-Registration

### Problem
When the user modifies the capture hotkey (e.g., from `CommandOrControl+Shift+X` to `Alt+Shift+C`), the new shortcut must become active immediately without requiring an application restart.

### Solution
1. Validate accelerator string syntax in domain (`validate_accelerator`).
2. Atomically update preferences in memory and write to `config.json`.
3. In `ManagePreferencesUseCase` / shortcut driver:
   - Call `driver.unregister_all()` (or `driver.unregister_shortcut(old_key)`).
   - Call `driver.register_shortcut(new_key, callback)`.
   - If registration fails (e.g. system conflict), rollback to old key and return error to the frontend.

---

## 3. Storage Directory & Format Settings

- `default_save_directory`: Optional string path. If `None` or empty, resolves to OS pictures directory (or downloads).
- `default_image_format`: Supported formats: `"png"` (lossless) and `"jpeg"` (compressed).
- `show_notifications`: Controls whether toast notifications appear after export.
- `copy_to_clipboard_on_capture`: Determines if capture is automatically copied upon completion.

---

## 4. Autostart & System Tray Integration

- Autostart flag persisted in `AppPreferences`.
- Tauri System Tray menu item "Preferencias..." emits an event or toggles the settings UI modal.
- Settings modal in UI: clean modal dialog matching corporate styling with tabs/sections for "General", "Atajos de Teclado", and "Almacenamiento".
