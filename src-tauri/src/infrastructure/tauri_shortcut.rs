use crate::ports::{IGlobalShortcutDriver, ShortcutDriverError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type ShortcutCallback = Arc<dyn Fn() + Send + Sync + 'static>;
type ShortcutMap = Arc<Mutex<HashMap<String, ShortcutCallback>>>;

#[derive(Default, Clone)]
pub struct TauriGlobalShortcutDriver {
    registered: ShortcutMap,
}

impl TauriGlobalShortcutDriver {
    pub fn new() -> Self {
        Self {
            registered: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl IGlobalShortcutDriver for TauriGlobalShortcutDriver {
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
}
