# Tauri IPC Command Contracts

**Feature**: `001-scaffolding-clean-architecture`  
**Date**: 2026-11-09  
**Status**: Formal Specification  

These contracts specify the typed inter-process communication (IPC) commands available between the frontend Webview (presentation) and the Tauri Rust backend.

---

## 1. Command: `get_app_info`

Retrieves basic application metadata and environment status.

- **Invoke Identifier**: `plugin:app|get_app_info` or `get_app_info`
- **Request Parameters**: None
- **Response Format**:
  ```json
  {
    "name": "capturador-pantalla",
    "version": "1.0.0",
    "environment": "development | production",
    "os": "macos | windows | linux",
    "local_first_verified": true
  }
  ```

---

## 2. Command: `normalize_bounding_box`

Executes pure domain normalization on arbitrary raw drag points.

- **Invoke Identifier**: `normalize_bounding_box`
- **Request Payload**:
  ```json
  {
    "p1": { "x": 500.0, "y": 400.0 },
    "p2": { "x": 100.0, "y": 50.0 },
    "scale_factor": 2.0
  }
  ```
- **Response Payload**:
  ```json
  {
    "origin": { "x": 100.0, "y": 50.0 },
    "size": { "width": 400.0, "height": 350.0 },
    "scale_factor": 2.0,
    "physical_rect": {
      "x": 200,
      "y": 100,
      "width": 800,
      "height": 700
    }
  }
  ```

---

## 3. Command: `get_session_state`

Returns the current lifecycle state of the capture session.

- **Invoke Identifier**: `get_session_state`
- **Request Parameters**: None
- **Response Payload**:
  ```json
  {
    "state": "Idle | Selecting | Completed | Cancelled"
  }
  ```
