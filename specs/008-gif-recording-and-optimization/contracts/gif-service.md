# Contract: GIF Export IPC Service (Feature 008)

## 1. IPC Commands

### `export_frames_to_gif`

Receives a sequence of raw RGBA frames with dimensions and fps, encodes an animated GIF, and saves it to local disk.

- **Request Payload**:
```json
{
  "width": 400,
  "height": 300,
  "framesRgba": [
    [255, 0, 0, 255, ...],
    [0, 255, 0, 255, ...]
  ],
  "fps": 15,
  "targetDirectory": "/Users/user/Pictures"
}
```

- **Response**:
```json
{
  "file_path": "/Users/user/Pictures/Animacion_2026-11-09_14-45-00.gif",
  "duration_seconds": 2,
  "file_size_bytes": 142050,
  "frame_count": 30
}
```
