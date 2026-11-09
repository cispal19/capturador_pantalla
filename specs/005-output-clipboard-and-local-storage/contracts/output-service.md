# Contract: Output Pipeline & Storage Service

**Feature**: `005-output-clipboard-and-local-storage`  
**Date**: 2026-11-09  
**Status**: Formal Specification

---

## 1. IPC Commands

### 1.1 `copy_capture_to_clipboard`
Writes composed RGBA pixels directly to OS system clipboard and resets session state to `Idle`.

- **Direction**: Frontend -> Backend
- **Command Name**: `copy_capture_to_clipboard`
- **Request Payload**:
  ```json
  {
    "width": 640,
    "height": 480,
    "rgba_data": [255, 0, 0, 255, ...]
  }
  ```
- **Response Payload**: `Result<bool, String>`

---

### 1.2 `save_capture_to_file`
Encodes and saves the image to a specified or default local directory with formatted timestamp filename.

- **Direction**: Frontend -> Backend
- **Command Name**: `save_capture_to_file`
- **Request Payload**:
  ```json
  {
    "width": 640,
    "height": 480,
    "rgba_data": [255, 0, 0, 255, ...],
    "format": "png",
    "target_directory": null
  }
  ```
- **Response Payload**:
  ```json
  {
    "file_path": "/Users/user/Pictures/Captura_2026-11-09_12-30-00.png",
    "file_size_bytes": 104230,
    "format": "png"
  }
  ```