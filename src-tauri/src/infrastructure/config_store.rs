use crate::domain::AppPreferences;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct JsonConfigStoreDriver {
    custom_path: Option<PathBuf>,
}

impl Default for JsonConfigStoreDriver {
    fn default() -> Self {
        Self::new(None)
    }
}

impl JsonConfigStoreDriver {
    pub fn new(custom_path: Option<PathBuf>) -> Self {
        Self { custom_path }
    }

    pub fn get_config_path(&self) -> PathBuf {
        if let Some(ref p) = self.custom_path {
            return p.clone();
        }
        Self::resolve_default_config_path()
    }

    pub fn resolve_default_config_path() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            if let Ok(app_data) = env::var("APPDATA") {
                return Path::new(&app_data)
                    .join("capturador-pantalla")
                    .join("config.json");
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
                if !xdg.trim().is_empty() {
                    return Path::new(&xdg)
                        .join("capturador-pantalla")
                        .join("config.json");
                }
            }

            if let Ok(home) = env::var("HOME") {
                if !home.trim().is_empty() {
                    return Path::new(&home)
                        .join(".config")
                        .join("capturador-pantalla")
                        .join("config.json");
                }
            }
        }

        env::temp_dir()
            .join("capturador-pantalla")
            .join("config.json")
    }

    pub fn load(&self) -> AppPreferences {
        let path = self.get_config_path();

        if !path.exists() {
            let defaults = AppPreferences::default();
            let _ = self.save(&defaults);
            return defaults;
        }

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<AppPreferences>(&content) {
                Ok(prefs) => {
                    if prefs.validate().is_ok() {
                        prefs
                    } else {
                        let defaults = AppPreferences::default();
                        let _ = self.save(&defaults);
                        defaults
                    }
                }
                Err(_) => {
                    // Corrupted or invalid JSON, recover by saving safe defaults
                    let defaults = AppPreferences::default();
                    let _ = self.save(&defaults);
                    defaults
                }
            },
            Err(_) => AppPreferences::default(),
        }
    }

    pub fn save(&self, prefs: &AppPreferences) -> Result<(), String> {
        prefs.validate()?;

        let path = self.get_config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let json = serde_json::to_string_pretty(prefs).map_err(|e| e.to_string())?;

        fs::write(&path, json).map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_resolve_default_config_path() {
        let path = JsonConfigStoreDriver::resolve_default_config_path();
        assert!(path.to_string_lossy().ends_with("config.json"));
    }

    #[test]
    fn test_config_store_load_default_when_file_not_exists() {
        let temp_dir = env::temp_dir().join("test_prefs_nonexistent");
        let _ = fs::remove_dir_all(&temp_dir);
        let config_file = temp_dir.join("config.json");

        let store = JsonConfigStoreDriver::new(Some(config_file.clone()));
        let prefs = store.load();

        assert_eq!(prefs, AppPreferences::default());
        assert!(config_file.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_config_store_save_and_load_roundtrip() {
        let temp_dir = env::temp_dir().join("test_prefs_roundtrip");
        let config_file = temp_dir.join("config.json");

        let store = JsonConfigStoreDriver::new(Some(config_file.clone()));
        let custom_prefs = AppPreferences {
            capture_shortcut: "Ctrl+Alt+S".to_string(),
            default_save_directory: Some("/custom/path".to_string()),
            default_image_format: "jpeg".to_string(),
            autostart_enabled: true,
            show_notifications: false,
            copy_to_clipboard_on_capture: false,
        };

        store.save(&custom_prefs).unwrap();
        let loaded = store.load();

        assert_eq!(loaded, custom_prefs);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_config_store_corrupted_file_recovers_to_default() {
        let temp_dir = env::temp_dir().join("test_prefs_corrupt");
        let config_file = temp_dir.join("config.json");
        let _ = fs::create_dir_all(&temp_dir);
        fs::write(&config_file, "{ corrupted_invalid_json !!!").unwrap();

        let store = JsonConfigStoreDriver::new(Some(config_file.clone()));
        let loaded = store.load();

        assert_eq!(loaded, AppPreferences::default());

        // File should have been healed
        let healed_content = fs::read_to_string(&config_file).unwrap();
        assert!(serde_json::from_str::<AppPreferences>(&healed_content).is_ok());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_config_store_rejects_invalid_preferences() {
        let temp_dir = env::temp_dir().join("test_prefs_invalid");
        let config_file = temp_dir.join("config.json");

        let store = JsonConfigStoreDriver::new(Some(config_file.clone()));
        let invalid_prefs = AppPreferences {
            capture_shortcut: "".to_string(),
            default_save_directory: None,
            default_image_format: "png".to_string(),
            autostart_enabled: false,
            show_notifications: true,
            copy_to_clipboard_on_capture: true,
        };

        assert!(store.save(&invalid_prefs).is_err());
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
