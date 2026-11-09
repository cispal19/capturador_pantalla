# Tauri IPC Command Contracts: Screen Capture & Shortcuts

**Feature**: `002-native-capture-and-global-shortcuts`  
**Date**: 2026-11-09  
**Status**: Formal Specification  

---

## 1. Command: `get_connected_monitors`

Enumerates all connected physical displays and their dimensional/scaling properties.

- **Invoke Identifier**: `get_connected_monitors`
- **Request Parameters**: None
- **Response Format**:
  ```json
  [
    {
      "id": 0,
      "name": "Built-in Retina Display",
      "logical_width": 1728,
      "logical_height": 1117,
      "physical_width": 3456,
      "physical_height": 2234,
      "scale_factor": 2.0,
      "is_primary": true
    }
  ]
  ```

---

## 2. Command: `trigger_screen_capture`

Manually triggers the screen capture workflow (e.g. from System Tray or button).

- **Invoke Identifier**: `trigger_screen_capture`
- **Request Parameters**:
  ```json
  {
    "display_id": 0 // optional: defaults to active display
  }
  ```
- **Response Format**:
  ```json
  {
    "success": true,
    "display_id": 0,
    "width": 3456,
    "height": 2234,
    "scale_factor": 2.0,
    "session_state": "Selecting"
  }
  ```

---

## 3. Command: `register_capture_shortcut`

Registers or updates the global capture shortcut.

- **Invoke Identifier**: `register_capture_shortcut`
- **Request Parameters**:
  ```json
  {
    "accelerator": "CommandOrControl+Shift+X"
  }
  ```
- **Response Format**:
  ```json
  {
    "registered": true,
    "accelerator": "CommandOrControl+Shift+X"
  }
  ```
