# Implementation Plan: 008-gif-recording-and-optimization

## 1. Technical Context

- **Platform Target**: Tauri 2.0, Rust 2021 edition, TypeScript 5.
- **GIF Standards**: GIF89a with looping Netscape application extension block.
- **Dependencies**: `image` crate with `features = ["png", "jpeg", "gif"]`.

---

## 2. Constitution & Quality Gates Evaluation

- **Principle I (TDD)**: Test-first development for all GIF config calculations, frame ingestion, and file emission.
- **Principle II (SOLID & Clean Architecture)**:
  - `domain::gif`: `GifExportConfig`, `GifExportResult`.
  - `infrastructure::storage`: Support for GIF encoding in `FileSystemStorageDriver` or GIF encoder helper.
  - `application::gif`: `ExportRecordingToGifUseCase`.
  - `presentation::commands`: `export_frames_to_gif` command.
  - `components/AnnotationToolbar`: "🎞️ GIF" export action button.
- **Principle III (Zero External Telemetry / Local-First)**: Processing and encoding occur completely on local hardware.

---

## 3. Layer Architecture & File Structure

```text
src-tauri/
├── Cargo.toml                       # Add "gif" to image crate features
├── src/
│   ├── domain/
│   │   ├── gif.rs                   # GifExportConfig, GifExportResult
│   │   └── mod.rs
│   ├── application/
│   │   ├── gif.rs                   # ExportRecordingToGifUseCase
│   │   └── mod.rs
│   ├── presentation/
│   │   ├── commands.rs              # export_frames_to_gif command
│   │   └── mod.rs
src/
├── services/
│   ├── tauriBridge.ts               # exportFramesToGif bridge method
│   └── tauriBridge.test.ts
├── types/
│   └── gif.ts                       # TypeScript interfaces
└── components/
    ├── AnnotationToolbar.ts         # Add "🎞️ GIF" button
    └── SelectionOverlay.ts          # Handle GIF export
```

---

## 4. TDD Execution Sequence

1. **Step 1 (Red)**: Add `"gif"` feature to `image` in `src-tauri/Cargo.toml` and write unit tests for `GifExportConfig` in `src-tauri/src/domain/gif.rs`.
2. **Step 2 (Green)**: Implement `GifExportConfig` in `src-tauri/src/domain/gif.rs` and export in `src-tauri/src/domain/mod.rs`.
3. **Step 3 (Red)**: Write unit tests for `ExportRecordingToGifUseCase` (encoding synthetic multi-frame RGBA into valid GIF89a file) in `src-tauri/src/application/gif.rs`.
4. **Step 4 (Green)**: Implement `ExportRecordingToGifUseCase` in `src-tauri/src/application/gif.rs` and export in `src-tauri/src/application/mod.rs`.
5. **Step 5 (Red & Green)**: Implement `export_frames_to_gif` IPC command in `src-tauri/src/presentation/commands.rs` and register in `src-tauri/src/lib.rs`.
6. **Step 6 (Red & Green)**: Add TypeScript interfaces in `src/types/gif.ts` and bridge method in `src/services/tauriBridge.ts`.
7. **Step 7 (Red & Green)**: Add "🎞️ GIF" button in `AnnotationToolbar.ts` and connect export in `SelectionOverlay.ts`.
8. **Step 8**: Full verification (`npm test`, `cargo clippy`, `cargo fmt`, `npm run format:check`).
