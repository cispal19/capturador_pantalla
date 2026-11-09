use crate::ports::{IGlobalShortcutDriver, ShortcutDriverError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type ShortcutCallback = Arc<dyn Fn() + Send + Sync + 'static>;
type ShortcutMap = Arc<Mutex<HashMap<String, ShortcutCallback>>>;

#[derive(Default, Clone)]
pub struct MockShortcutDriver {
    registered: ShortcutMap,
}

impl MockShortcutDriver {
    pub fn new() -> Self {
        Self {
            registered: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn trigger(&self, accelerator: &str) -> bool {
        let map = self.registered.lock().unwrap();
        if let Some(cb) = map.get(accelerator) {
            cb();
            true
        } else {
            false
        }
    }
}

impl IGlobalShortcutDriver for MockShortcutDriver {
    fn register_shortcut(
        &self,
        accelerator: &str,
        callback: Box<dyn Fn() + Send + Sync + 'static>,
    ) -> Result<(), ShortcutDriverError> {
        let mut map = self.registered.lock().unwrap();
        if map.contains_key(accelerator) {
            return Err(ShortcutDriverError::AlreadyRegistered(
                accelerator.to_string(),
            ));
        }
        map.insert(accelerator.to_string(), Arc::from(callback));
        Ok(())
    }

    fn is_registered(&self, accelerator: &str) -> bool {
        let map = self.registered.lock().unwrap();
        map.contains_key(accelerator)
    }

    fn unregister_all(&self) -> Result<(), ShortcutDriverError> {
        let mut map = self.registered.lock().unwrap();
        map.clear();
        Ok(())
    }

    fn unregister_shortcut(&self, accelerator: &str) -> Result<(), ShortcutDriverError> {
        let mut map = self.registered.lock().unwrap();
        map.remove(accelerator);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_mock_shortcut_registration_and_trigger() {
        let driver = MockShortcutDriver::new();
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = flag.clone();

        driver
            .register_shortcut(
                "CommandOrControl+Shift+X",
                Box::new(move || {
                    flag_clone.store(true, Ordering::SeqCst);
                }),
            )
            .unwrap();

        assert!(driver.is_registered("CommandOrControl+Shift+X"));
        assert!(!flag.load(Ordering::SeqCst));

        assert!(driver.trigger("CommandOrControl+Shift+X"));
        assert!(flag.load(Ordering::SeqCst));
    }

    #[test]
    fn test_mock_shortcut_duplicate_rejected() {
        let driver = MockShortcutDriver::new();
        driver.register_shortcut("Cmd+X", Box::new(|| {})).unwrap();

        let dup = driver.register_shortcut("Cmd+X", Box::new(|| {}));
        assert!(matches!(
            dup,
            Err(ShortcutDriverError::AlreadyRegistered(_))
        ));
    }

    #[test]
    fn test_mock_shortcut_unregister_all() {
        let driver = MockShortcutDriver::new();
        driver.register_shortcut("Cmd+1", Box::new(|| {})).unwrap();
        driver.register_shortcut("Cmd+2", Box::new(|| {})).unwrap();

        assert_eq!(driver.registered.lock().unwrap().len(), 2);
        driver.unregister_all().unwrap();
        assert_eq!(driver.registered.lock().unwrap().len(), 0);
    }
}
