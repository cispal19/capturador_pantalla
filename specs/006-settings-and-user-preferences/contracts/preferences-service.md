# Contract: Preferences IPC Service (Feature 006)

## 1. IPC Commands

### `get_preferences`

Retrieves current stored preferences, creating the default file if not existing.

- **Request**: No parameters.
- **Response**:
```json
{
  "capture_shortcut": "CommandOrControl+Shift+X",
  "default_save_directory": null,
  "default_image_format": "png",
  "autostart_enabled": false,
  "show_notifications": true,
  "copy_to_clipboard_on_capture": true
}
```

---

### `update_preferences`

Validates, persists, and dynamically applies updated user preferences (including shortcut re-registration).

- **Request Payload**:
```json
{
  "preferences": {
    "capture_shortcut": "Alt+Shift+C",
    "default_save_directory": "/Users/user/Pictures/Screenshots",
    "default_image_format": "jpeg",
    "autostart_enabled": true,
    "show_notifications": true,
    "copy_to_clipboard_on_capture": true
  }
}
```

- **Response**:
  - `Ok(AppPreferences)`: The updated and saved preferences object.
  - `Err(String)`: Validation error message or shortcut registration error.

---

### `reset_preferences_to_defaults`

Resets all preferences to factory defaults.

- **Request**: No parameters.
- **Response**:
  - `Ok(AppPreferences)`: Default preferences object.
