# Contract: Recording IPC Service (Feature 007)

## 1. IPC Commands

### `start_screen_recording`

Initiates a recording session on the specified screen region.

- **Request Payload**:
```json
{
  "x": 100,
  "y": 150,
  "width": 800,
  "height": 600,
  "fps": 30,
  "timeLimitSeconds": 600
}
```

- **Response**:
```json
{
  "state": "Recording",
  "region": {
    "x": 100,
    "y": 150,
    "width": 800,
    "height": 600,
    "fps": 30,
    "time_limit_seconds": 600
  }
}
```

---

### `pause_screen_recording`

Pauses an active recording session.

- **Request**: No parameters.
- **Response**:
```json
{
  "state": "Paused"
}
```

---

### `resume_screen_recording`

Resumes a paused recording session.

- **Request**: No parameters.
- **Response**:
```json
{
  "state": "Recording"
}
```

---

### `stop_screen_recording`

Stops recording, finalizes the video container, and persists the video file.

- **Request Payload**:
```json
{
  "videoData": [26, 69, 223, 163, ...],
  "durationSeconds": 14,
  "format": "webm"
}
```

- **Response**:
```json
{
  "filePath": "/Users/user/Pictures/Grabacion_2026-11-09_14-30-00.webm",
  "durationSeconds": 14,
  "fileSizeBytes": 1542030,
  "format": "webm",
  "fps": 30
}
```

---

### `cancel_screen_recording`

Discards all recorded buffer data and transitions back to `Idle`.

- **Request**: No parameters.
- **Response**:
```json
{
  "state": "Idle"
}
```

---

### `get_recording_status`

Queries current recording state, elapsed seconds, and active region.

- **Request**: No parameters.
- **Response**:
```json
{
  "state": "Recording",
  "elapsed_seconds": 12,
  "region": { ... }
}
```
