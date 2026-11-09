use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppPreferences {
    #[serde(default = "default_capture_shortcut")]
    pub capture_shortcut: String,

    #[serde(default)]
    pub default_save_directory: Option<String>,

    #[serde(default = "default_image_format")]
    pub default_image_format: String,

    #[serde(default)]
    pub autostart_enabled: bool,

    #[serde(default = "default_true")]
    pub show_notifications: bool,

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

impl AppPreferences {
    pub fn validate(&self) -> Result<(), String> {
        if self.capture_shortcut.trim().is_empty() {
            return Err("El atajo de captura no puede estar vacío".to_string());
        }

        let fmt = self.default_image_format.to_lowercase();
        if fmt != "png" && fmt != "jpeg" && fmt != "jpg" {
            return Err(format!(
                "Formato de imagen no soportado '{}'. Solo 'png' o 'jpeg'",
                self.default_image_format
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_default_preferences() {
        let prefs = AppPreferences::default();
        assert!(!prefs.capture_shortcut.is_empty());
        assert_eq!(prefs.default_image_format, "png");
        assert!(prefs.default_save_directory.is_none());
        assert!(!prefs.autostart_enabled);
        assert!(prefs.show_notifications);
        assert!(prefs.copy_to_clipboard_on_capture);
        assert!(prefs.validate().is_ok());
    }

    #[test]
    fn test_preferences_validation() {
        let mut prefs = AppPreferences::default();
        prefs.capture_shortcut = "   ".to_string();
        assert!(prefs.validate().is_err());

        prefs.capture_shortcut = "Ctrl+Shift+X".to_string();
        prefs.default_image_format = "bmp".to_string();
        assert!(prefs.validate().is_err());

        prefs.default_image_format = "jpeg".to_string();
        assert!(prefs.validate().is_ok());
    }

    #[test]
    fn test_serde_roundtrip() {
        let prefs = AppPreferences {
            capture_shortcut: "Alt+Shift+S".to_string(),
            default_save_directory: Some("/custom/dir".to_string()),
            default_image_format: "jpeg".to_string(),
            autostart_enabled: true,
            show_notifications: false,
            copy_to_clipboard_on_capture: false,
        };

        let json = serde_json::to_string(&prefs).unwrap();
        let deserialized: AppPreferences = serde_json::from_str(&json).unwrap();
        assert_eq!(prefs, deserialized);
    }

    #[test]
    fn test_serde_partial_json_uses_defaults() {
        let json = "{}";
        let prefs: AppPreferences = serde_json::from_str(json).unwrap();
        assert_eq!(prefs, AppPreferences::default());
    }
}
