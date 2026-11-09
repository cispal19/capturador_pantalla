# Contract: Selection Overlay IPC & Application Service

**Feature**: `003-interactive-selection-overlay`  
**Date**: 2026-11-09  
**Status**: Formal Specification

---

## 1. IPC Commands (Frontend <-> Backend)

### 1.1 `cancel_capture_session`
Aborts an in-flight capture session immediately, resets the backend session manager to `Idle`, and hides/closes the overlay window.

- **Direction**: Frontend -> Backend
- **Command Name**: `cancel_capture_session`
- **Request Payload**: None
- **Response Payload**: `Result<CaptureSessionState, String>`
- **Behavior**:
  - Sets application session state to `Idle`.
  - Discards any buffered display image in memory.
  - Returns `CaptureSessionState::Idle`.

---

### 1.2 `commit_selection_region`
Commits the final user-selected bounding box into the session manager, transitioning state to `Annotating` (or ready for export in Phase 1).

- **Direction**: Frontend -> Backend
- **Command Name**: `commit_selection_region`
- **Request Payload**:
  ```json
  {
    "origin_x": 100.0,
    "origin_y": 150.0,
    "width": 640.0,
    "height": 480.0,
    "scale_factor": 2.0
  }
  ```
- **Response Payload**: `Result<BoundingBox, String>`
- **Error Conditions**:
  - `origin_x < 0.0` or `origin_y < 0.0`
  - `width < 10.0` or `height < 10.0`
  - Session is not currently in `Selecting` state.

---

### 1.3 `get_selection_handles`
Pure mathematical evaluation of the 8 handle anchor positions for a given bounding box.

- **Direction**: Frontend -> Backend (or executed in frontend domain library)
- **Command Name**: `get_selection_handles`
- **Request Payload**:
  ```json
  {
    "origin_x": 100.0,
    "origin_y": 150.0,
    "width": 640.0,
    "height": 480.0
  }
  ```
- **Response Payload**:
  ```json
  [
    { "handle": "NorthWest", "x": 100.0, "y": 150.0, "cursor": "nwse-resize" },
    { "handle": "North", "x": 420.0, "y": 150.0, "cursor": "ns-resize" },
    { "handle": "NorthEast", "x": 740.0, "y": 150.0, "cursor": "nesw-resize" },
    { "handle": "East", "x": 740.0, "y": 390.0, "cursor": "ew-resize" },
    { "handle": "SouthEast", "x": 740.0, "y": 630.0, "cursor": "nwse-resize" },
    { "handle": "South", "x": 420.0, "y": 630.0, "cursor": "ns-resize" },
    { "handle": "SouthWest", "x": 100.0, "y": 630.0, "cursor": "nesw-resize" },
    { "handle": "West", "x": 100.0, "y": 390.0, "cursor": "ew-resize" }
  ]
  ```

---

## 2. Event Listeners (Tauri Windows)

### 2.1 `session-cancelled`
Emitted by backend if the session is cancelled via global shortcut or system tray.

- **Channel**: `capture://session-cancelled`
- **Payload**: None
- **Frontend Action**: Canvas overlay clears selection, resets state to `idle`, and hides window.