use crate::domain::AppPreferences;
use crate::infrastructure::JsonConfigStoreDriver;
use crate::ports::IGlobalShortcutDriver;
use std::sync::{Arc, Mutex};

pub type ShortcutTriggerCallback = Arc<dyn Fn() + Send + Sync + 'static>;

#[derive(Clone)]
pub struct ManagePreferencesUseCase {
    store: Arc<JsonConfigStoreDriver>,
    shortcut_driver: Option<Arc<dyn IGlobalShortcutDriver>>,
    trigger_callback: Arc<Mutex<Option<ShortcutTriggerCallback>>>,
}

impl ManagePreferencesUseCase {
    pub fn new(
        store: Arc<JsonConfigStoreDriver>,
        shortcut_driver: Option<Arc<dyn IGlobalShortcutDriver>>,
    ) -> Self {
        Self {
            store,
            shortcut_driver,
            trigger_callback: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_trigger_callback(&self, callback: ShortcutTriggerCallback) {
        let mut guard = self.trigger_callback.lock().unwrap();
        *guard = Some(callback);
    }

    pub fn get_preferences(&self) -> AppPreferences {
        self.store.load()
    }

    pub fn update_preferences(&self, new_prefs: AppPreferences) -> Result<AppPreferences, String> {
        new_prefs.validate()?;

        let current_prefs = self.store.load();

        if let Some(ref driver) = self.shortcut_driver {
            if current_prefs.capture_shortcut != new_prefs.capture_shortcut {
                let cb_opt = self.trigger_callback.lock().unwrap().clone();
                let cb = cb_opt.unwrap_or_else(|| Arc::new(|| {}));

                // Register new shortcut first to ensure it succeeds
                let cb_clone = cb.clone();
                driver
                    .register_shortcut(&new_prefs.capture_shortcut, Box::new(move || cb_clone()))
                    .map_err(|e| {
                        format!(
                            "Error al registrar atajo {}: {}",
                            new_prefs.capture_shortcut, e
                        )
                    })?;

                // Then unregister old shortcut
                let _ = driver.unregister_shortcut(&current_prefs.capture_shortcut);
            }
        }

        self.store.save(&new_prefs)?;
        Ok(new_prefs)
    }

    pub fn reset_to_defaults(&self) -> Result<AppPreferences, String> {
        let defaults = AppPreferences::default();
        self.update_preferences(defaults)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::infrastructure::MockShortcutDriver;
    use std::env;
    use std::fs;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_get_preferences_returns_defaults() {
        let temp_dir = env::temp_dir().join("test_app_prefs_get");
        let _ = fs::remove_dir_all(&temp_dir);
        let config_file = temp_dir.join("config.json");

        let store = Arc::new(JsonConfigStoreDriver::new(Some(config_file)));
        let use_case = ManagePreferencesUseCase::new(store, None);

        let prefs = use_case.get_preferences();
        assert_eq!(prefs, AppPreferences::default());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_update_preferences_valid_and_reset() {
        let temp_dir = env::temp_dir().join("test_app_prefs_update");
        let _ = fs::remove_dir_all(&temp_dir);
        let config_file = temp_dir.join("config.json");

        let store = Arc::new(JsonConfigStoreDriver::new(Some(config_file)));
        let use_case = ManagePreferencesUseCase::new(store, None);

        let mut custom = AppPreferences::default();
        custom.default_image_format = "jpeg".to_string();
        custom.autostart_enabled = true;
        custom.show_notifications = false;

        let updated = use_case.update_preferences(custom.clone()).unwrap();
        assert_eq!(updated, custom);

        let read_back = use_case.get_preferences();
        assert_eq!(read_back, custom);

        // Reset to defaults
        let reset = use_case.reset_to_defaults().unwrap();
        assert_eq!(reset, AppPreferences::default());
        assert_eq!(use_case.get_preferences(), AppPreferences::default());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_update_preferences_dynamic_shortcut_reregistration() {
        let temp_dir = env::temp_dir().join("test_app_prefs_reregister");
        let _ = fs::remove_dir_all(&temp_dir);
        let config_file = temp_dir.join("config.json");

        let store = Arc::new(JsonConfigStoreDriver::new(Some(config_file)));
        let shortcut_driver = Arc::new(MockShortcutDriver::new());

        let fired = Arc::new(AtomicBool::new(false));
        let fired_clone = fired.clone();

        let use_case =
            ManagePreferencesUseCase::new(store, Some(shortcut_driver.clone() as Arc<_>));
        use_case.set_trigger_callback(Arc::new(move || {
            fired_clone.store(true, Ordering::SeqCst);
        }));

        let initial_shortcut = use_case.get_preferences().capture_shortcut;

        // Register initial shortcut
        shortcut_driver
            .register_shortcut(&initial_shortcut, Box::new(|| {}))
            .unwrap();
        assert!(shortcut_driver.is_registered(&initial_shortcut));

        // Now update shortcut to Alt+Shift+K
        let mut new_prefs = use_case.get_preferences();
        new_prefs.capture_shortcut = "Alt+Shift+K".to_string();

        let updated = use_case.update_preferences(new_prefs).unwrap();
        assert_eq!(updated.capture_shortcut, "Alt+Shift+K");

        // Old shortcut should be unregistered, new shortcut registered
        assert!(!shortcut_driver.is_registered(&initial_shortcut));
        assert!(shortcut_driver.is_registered("Alt+Shift+K"));

        // Trigger new shortcut
        assert!(shortcut_driver.trigger("Alt+Shift+K"));
        assert!(fired.load(Ordering::SeqCst));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_update_preferences_rejects_invalid_values() {
        let temp_dir = env::temp_dir().join("test_app_prefs_invalid");
        let _ = fs::remove_dir_all(&temp_dir);
        let config_file = temp_dir.join("config.json");

        let store = Arc::new(JsonConfigStoreDriver::new(Some(config_file)));
        let use_case = ManagePreferencesUseCase::new(store, None);

        let mut invalid = AppPreferences::default();
        invalid.capture_shortcut = "".to_string();

        assert!(use_case.update_preferences(invalid).is_err());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
