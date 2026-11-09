# Quickstart Validation Guide: Screen Capture Engine & Global Shortcuts

**Feature**: `002-native-capture-and-global-shortcuts`  
**Date**: 2026-11-09  
**Status**: Runnable Verification Guide  

---

## 1. Automated Unit & Driver Tests (Headless)

Verify that the domain entities, mock capture driver, and shortcut logic evaluate with 100% success without requiring a graphical display:

```bash
npm run test:backend
```

**Expected Outcome**:
```text
running 19 tests
test domain::display::tests::test_valid_display_info ... ok
test domain::display::tests::test_scale_factor_ratio ... ok
test infrastructure::mock_capture::tests::test_mock_screen_capture_gradient ... ok
test infrastructure::mock_capture::tests::test_mock_multi_monitor_enumeration ... ok
...
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 2. Frontend IPC Bridge Tests

Verify that the frontend services correctly invoke the new Tauri commands (`get_connected_monitors`, `trigger_screen_capture`, `register_capture_shortcut`):

```bash
npm run test:frontend
```

**Expected Outcome**: All vitest unit tests pass.

---

## 3. Interactive Smoke Test (Desktop Environment)

Launch the desktop application:
```bash
npm run tauri dev
```

1. **System Tray Verification**:
   - Confirm a tray icon appears in the system menu bar / taskbar.
   - Right-click the icon: verify "Tomar Captura" and "Salir" appear.
2. **Global Shortcut Verification**:
   - Focus any external window (e.g. browser, editor, terminal).
   - Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Windows/Linux).
   - Verify that the capture trigger is received and the capture session transitions to `Selecting`.
