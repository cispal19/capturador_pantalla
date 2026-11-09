# Contract: Annotation Studio & Obfuscation Service

**Feature**: `004-annotation-studio-and-obfuscation`  
**Date**: 2026-11-09  
**Status**: Formal Specification

---

## 1. IPC Commands

### 1.1 `apply_obfuscation`
Applies an irreversible pixelation filter directly to an in-memory display buffer region.

- **Direction**: Frontend -> Backend
- **Command Name**: `apply_obfuscation`
- **Request Payload**:
  ```json
  {
    "rect_x": 100,
    "rect_y": 150,
    "width": 200,
    "height": 50,
    "block_size": 12
  }
  ```
- **Response Payload**: `Result<bool, String>`

---

### 1.2 `sync_annotations`
Synchronizes the list of vector annotations to the backend capture session state before export.

- **Direction**: Frontend -> Backend
- **Command Name**: `sync_annotations`
- **Request Payload**:
  ```json
  {
    "annotations": [
      {
        "type": "rectangle",
        "id": "anno-1",
        "x": 100.0,
        "y": 100.0,
        "width": 200.0,
        "height": 150.0,
        "color": "#ef4444",
        "stroke_width": 4.0
      }
    ]
  }
  ```
- **Response Payload**: `Result<usize, String>` (returns count of synchronized annotations)