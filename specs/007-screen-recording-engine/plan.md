# Implementation Plan: 007-screen-recording-engine

## 1. Technical Context

- **Platform Target**: Tauri 2.0 (macOS, Linux, Windows), Local-First (BR-001), Paridad de Plataforma (BR-007).
- **Video Encoders & Containers**: WebM (VP8/VP9) and MP4.
- **Layers**: Clean Architecture (Domain, Application, Ports, Infrastructure, Presentation).

---

## 2. Constitution & Quality Gates Evaluation

- **Principle I (TDD)**: Mandatory test-first for all recording value objects, use cases, state transitions, and UI components.
- **Principle II (SOLID & Clean Architecture)**:
  - `domain::recording`: `RecordingState`, `RecordingRegion`, `RecordingMetadata`.
  - `ports::IScreenRecorderDriver`: Recording contract.
  - `infrastructure::mock_recorder`: Test recorder driver.
  - `application::recording`: `RecordingSessionManager` and recording use cases (`Start`, `Pause`, `Resume`, `Stop`, `Cancel`).
  - `presentation::commands`: IPC endpoints for recording lifecycle.
  - `src/components/RecordingToolbar`: Floating UI controls with timer and visual states.
- **Principle III (Zero External Telemetry / Local-First)**: Video files encoded and saved 100% locally on device.

---

## 3. Layer Architecture & File Structure

```text
src-tauri/
├── src/
│   ├── domain/
│   │   ├── recording.rs             # RecordingState, RecordingRegion, RecordingMetadata
│   │   └── mod.rs
│   ├── ports/
│   │   └── mod.rs                  # IScreenRecorderDriver trait
│   ├── infrastructure/
│   │   ├── mock_recorder.rs         # MockScreenRecorderDriver
│   │   └── mod.rs
│   ├── application/
│   │   ├── recording.rs             # RecordingSessionManager, use cases
│   │   └── mod.rs
│   ├── presentation/
│   │   ├── commands.rs              # Recording IPC commands
│   │   └── mod.rs
src/
├── components/
│   ├── RecordingToolbar.ts          # Floating controls: Timer, Pause, Stop, Cancel
│   └── RecordingToolbar.test.ts     # Component unit tests
├── services/
│   ├── tauriBridge.ts               # Recording bridge methods
│   └── tauriBridge.test.ts
└── types/
    └── recording.ts                 # TypeScript types
```

---

## 4. TDD Execution Sequence

1. **Step 1 (Red)**: Write unit tests for `RecordingRegion` (validation, even-dimension alignment, time limit clamping) and `RecordingState` in `src-tauri/src/domain/recording.rs`.
2. **Step 2 (Green)**: Implement `RecordingRegion` and `RecordingState` in `src-tauri/src/domain/recording.rs` and export in `src-tauri/src/domain/mod.rs`.
3. **Step 3 (Red)**: Write unit tests for `MockScreenRecorderDriver` in `src-tauri/src/infrastructure/mock_recorder.rs`.
4. **Step 4 (Green)**: Implement `MockScreenRecorderDriver` and export in `src-tauri/src/infrastructure/mod.rs`.
5. **Step 5 (Red)**: Write unit tests for `RecordingSessionManager` (lifecycle: Idle -> Recording -> Paused -> Finalizing -> Idle) in `src-tauri/src/application/recording.rs`.
6. **Step 6 (Green)**: Implement `RecordingSessionManager` in `src-tauri/src/application/recording.rs` and export in `src-tauri/src/application/mod.rs`.
7. **Step 7 (Red & Green)**: Implement recording IPC commands in `src-tauri/src/presentation/commands.rs` with tests and register in `src-tauri/src/lib.rs`.
8. **Step 8 (Red & Green)**: Define TypeScript interfaces in `src/types/recording.ts` and add bridge methods to `src/services/tauriBridge.ts`.
9. **Step 9 (Red & Green)**: Implement `RecordingToolbar.ts` with tests in `src/components/RecordingToolbar.test.ts`.
10. **Step 10**: Integrate recording toolbar into `SelectionOverlay.ts` and add mode toggle.
11. **Step 11**: Full verification (`npm test`, `cargo clippy`, `cargo fmt`, `npm run format:check`).
