# Capture Service Port & Application Contracts

**Feature**: `002-native-capture-and-global-shortcuts`  
**Date**: 2026-11-09  
**Status**: Formal Specification  

---

## 1. Monitor Discovery & Capture Port (`IScreenCaptureDriver`)

```rust
use crate::domain::{
    buffer::RawDisplayBuffer,
    DisplayInfo,
};
use crate::ports::CaptureDriverError;

pub trait IScreenCaptureDriver: Send + Sync {
    /// Enumerate all connected physical displays
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, CaptureDriverError>;

    /// Capture raw pixel buffer from primary/active display
    fn capture_active_display(&self) -> Result<RawDisplayBuffer, CaptureDriverError>;

    /// Capture raw pixel buffer from a specific display by its ID
    fn capture_display_by_id(&self, display_id: u32) -> Result<RawDisplayBuffer, CaptureDriverError>;

    /// Capture raw pixel buffers for all connected displays
    fn capture_all_displays(&self) -> Result<Vec<(DisplayInfo, RawDisplayBuffer)>, CaptureDriverError>;
}
```

---

## 2. Global Shortcut Driver Port (`IGlobalShortcutDriver`)

```rust
use crate::ports::ShortcutDriverError;

pub trait IGlobalShortcutDriver: Send + Sync {
    /// Register accelerator hotkey with trigger callback
    fn register_shortcut(
        &self,
        accelerator: &str,
        callback: Box<dyn Fn() + Send + Sync + 'static>,
    ) -> Result<(), ShortcutDriverError>;

    /// Check if shortcut is currently active
    fn is_registered(&self, accelerator: &str) -> bool;

    /// Unregister all active hotkeys
    fn unregister_all(&self) -> Result<(), ShortcutDriverError>;
}
```
