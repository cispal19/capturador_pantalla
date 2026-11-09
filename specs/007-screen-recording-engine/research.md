# Research: Screen Recording Engine (Feature 007)

## 1. Technical Context & Cross-Platform Video Encoding

### Decision: WebM / MP4 Local Video Generation
- **Target Formats**:
  - **WebM (VP8/VP9)**: Royalty-free, cross-platform standard supported natively on macOS, Linux, and Windows, ideal for corporate messaging (Slack, Microsoft Teams, GitHub, Jira).
  - **MP4 (H.264)**: Standard container where supported.
- **Architecture Strategy**:
  - **Backend (Rust)**:
    - Implements `IScreenRecorderDriver` and `RecordingSessionManager`.
    - Manages recording state machine (`Idle` -> `Recording` <-> `Paused` -> `Finalizing`).
    - Validates recording regions (clamping, ensuring even width/height dimensions).
    - Enforces time limits (`time_limit_seconds`, default 600s / 10 min).
    - Receives final video binary data, writes to disk using `FileSystemStorageDriver` with naming `Grabacion_YYYY-MM-DD_HH-mm-ss.[ext]`.
  - **Frontend (Canvas / Webview)**:
    - `RecordingToolbar`: floating widget positioned adjacent to the active selection box with Live Timer, Pause/Resume, Stop, and Cancel buttons.
    - `RecordingBorder`: pulsing red border (or amber in paused state) rendering the active recording frame.
    - Video frame streaming using webview hardware acceleration / `MediaRecorder` with fallback chunk accumulator.

---

## 2. State Machine Transitions

```text
       ┌────────┐
       │  Idle  │◄───────────────────┐
       └───┬────┘                    │
           │ start_recording()       │ cancel_recording()
           ▼                         │
     ┌───────────┐                   │
┌───►│ Recording │───────────────────┤
│    └─────┬─────┘                   │
│          │ pause_recording()       │
│          ▼                         │
│    ┌───────────┐                   │
└─── │  Paused   │───────────────────┤
     └───────────┘                   │
           │ stop_recording()        │
           ▼                         │
     ┌───────────┐                   │
     │Finalizing │───────────────────┘
     └───────────┘
```

---

## 3. Dimensional Constraints (Even Width & Height)

Video encoders (H.264, VP8, VP9) require dimensions to be multiples of 2.
- The `RecordingRegion` value object automatically rounds dimensions down to the nearest even integer:
  ```rust
  let even_width = width & !1;
  let even_height = height & !1;
  ```
- Sub-minimum dimensions (< 50x50 px) are rejected with a descriptive error.
