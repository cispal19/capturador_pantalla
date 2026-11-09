# Data Model: User Preferences & Settings (Feature 006)

## 1. Domain Entities & Value Objects (Rust & TypeScript)

### Rust Domain: `AppPreferences`

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppPreferences {
    /// Global hotkey string for screen capture (e.g., "CommandOrControl+Shift+X")
    #[serde(default = "default_capture_shortcut")]
    pub capture_shortcut: String,

    /// Optional target folder path for automatic local saving
    #[serde(default)]
    pub default_save_directory: Option<String>,

    /// Default output format: "png" or "jpeg"
    #[serde(default = "default_image_format")]
    pub default_image_format: String,

    /// Whether the application should register for OS autostart
    #[serde(default)]
    pub autostart_enabled: bool,

    /// Whether to show toast notifications upon export
    #[serde(default = "default_true")]
    pub show_notifications: bool,

    /// Whether to copy to clipboard upon completion
    #[serde(default = "default_true")]
    pub copy_to_clipboard_on_capture: bool,
}

fn default_capture_shortcut() -> String {
    #[cfg(target_os = "macos")]
    {
        "CommandOrControl+Shift+X".to_string()
    }
    #[cfg(not(target_os = "macos"))]
    {
        "PrintScreen".to_string()
    }
}

fn default_image_format() -> String {
    "png".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            capture_shortcut: default_capture_shortcut(),
            default_save_directory: None,
            default_image_format: default_image_format(),
            autostart_enabled: false,
            show_notifications: true,
            copy_to_clipboard_on_capture: true,
        }
    }
}
```

---

### TypeScript Interface: `AppPreferences`

```typescript
export interface AppPreferences {
  capture_shortcut: string;
  default_save_directory?: string | null;
  default_image_format: "png" | "jpeg";
  autostart_enabled: boolean;
  show_notifications: boolean;
  copy_to_clipboard_on_capture: boolean;
}

export const DEFAULT_PREFERENCES: AppPreferences = {
  capture_shortcut: "CommandOrControl+Shift+X",
  default_save_directory: null,
  default_image_format: "png",
  autostart_enabled: false,
  show_notifications: true,
  copy_to_clipboard_on_capture: true,
};
```

---

## 2. Validation Rules

- `capture_shortcut`:
  - Must not be empty.
  - Must contain valid key tokens (e.g. `CommandOrControl`, `Ctrl`, `Alt`, `Shift`, `Super`, alphanumeric or function key).
- `default_image_format`:
  - Must be either `"png"` or `"jpeg"`.
- `default_save_directory`:
  - If provided, must be a syntactically valid path.
