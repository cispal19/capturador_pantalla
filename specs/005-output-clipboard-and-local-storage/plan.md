# Implementation Plan: Output Pipeline, Clipboard & Local Storage

**Branch**: `005-output-clipboard-and-local-storage` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/005-output-clipboard-and-local-storage/spec.md`

---

## Summary

Implement the export pipeline for captured and annotated screenshots. This enables the user to press `Enter` or click "Copiar" to write the composite image directly to the OS clipboard (`image/png`) via `arboard`, and press `Ctrl+S` / `Cmd+S` or click "Guardar" to write encoded PNG/JPEG/WebP images to the local file system with clean ISO timestamp naming (`Captura_YYYY-MM-DD_HH-mm-ss.[ext]`), closing the overlay automatically.

---

## Technical Context

**Language/Version**: Rust 1.77+ (Backend) / TypeScript 5.3+ (Frontend)  
**Primary Dependencies**: `arboard = "3"`, `image = "0.25"`, Tauri 2.0, Vitest, Cargo  
**Storage**: Local file system (Pictures / Downloads), zero cloud  
**Testing**: `cargo test` (isolated headless tests with mocks and memory buffers), `vitest` (frontend unit tests)  
**Target Platform**: macOS, Windows, Linux  
**Performance Goals**: Clipboard write in < 100ms; disk encode & write in < 150ms for 1080p/4K  
**Constraints**: 100% Local-First; zero network telemetry; atomic buffer management  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Plan Compliance | Status |
| :--- | :--- | :--- | :--- |
| **Principle I: TDD** | Tests written first in Red state | `FileNameGenerator`, `MockClipboardDriver`, `FileSystemStorageDriver`, and export use cases tested before integration. | **PASS** |
| **Principle II: SOLID** | Single Responsibility & Interface Segregation | `IClipboardDriver` separated from `IFileSystemDriver`; distinct use cases for clipboard vs file export. | **PASS** |
| **Principle III: Clean Architecture** | Inversion of Control | Application use cases depend purely on port traits (`ports::IClipboardDriver`, `ports::IFileSystemDriver`). | **PASS** |
| **Principle IV: DRY & KISS** | Minimal necessary complexity | In-memory raw buffer reuse; no temporary `/tmp` files needed. | **PASS** |
| **Principle V: Dependency Management** | Vetted libraries | `arboard` (MIT/Apache) and `image` (MIT/Apache); zero critical vulnerabilities. | **PASS** |
| **Principle VI: Security & Governance** | Zero data exfiltration | Local-first guaranteed; writes strictly to system clipboard and local user storage. | **PASS** |

---

## Project Structure

### Documentation

```text
specs/005-output-clipboard-and-local-storage/
├── spec.md              # Feature specification
├── plan.md              # Implementation plan
├── research.md          # Clipboard and storage architecture research
├── data-model.md        # Export data transfer objects and filename generator
├── contracts/           # Output IPC service contract
│   └── output-service.md
├── quickstart.md        # Test & manual validation guide
└── tasks.md             # TDD tasks list
```

### Source Code

```text
capturador_pantalla/
├── src-tauri/src/
│   ├── domain/
│   │   ├── naming.rs           # Added: FileNameGenerator
│   │   └── mod.rs              # Export FileNameGenerator
│   ├── application/
│   │   ├── export.rs           # Added: ExportToClipboardUseCase, ExportToFileUseCase
│   │   └── mod.rs              # Export export use cases
│   ├── infrastructure/
│   │   ├── clipboard.rs        # Added: ArboardClipboardDriver & MockClipboardDriver
│   │   ├── storage.rs          # Added: FileSystemStorageDriver & MockStorageDriver
│   │   └── mod.rs              # Export infrastructure adapters
│   ├── presentation/
│   │   └── commands.rs         # Added: copy_capture_to_clipboard, save_capture_to_file
│   └── lib.rs                  # Register IPC commands
├── src/
│   ├── components/
│   │   ├── AnnotationToolbar.ts      # Added: "Copiar" and "Guardar" buttons
│   │   └── SelectionOverlay.ts       # Added: Enter & Cmd+S keyboard listeners
│   └── services/
│       ├── tauriBridge.ts      # Added: copyCaptureToClipboard, saveCaptureToFile
│       └── tauriBridge.test.ts # Bridge tests
```

---

## Architecture & Implementation Flow

### Phase 1: Infrastructure & Domain Foundations (TDD)
1. Add `arboard` dependency to `src-tauri/Cargo.toml`.
2. Write unit tests for `FileNameGenerator` in `src-tauri/src/domain/naming.rs` and implement.
3. Write unit tests for `MockClipboardDriver` and `ArboardClipboardDriver` in `src-tauri/src/infrastructure/clipboard.rs`.
4. Write unit tests for `FileSystemStorageDriver` in `src-tauri/src/infrastructure/storage.rs`.

### Phase 2: Application Use Cases & IPC Commands (TDD)
1. Write unit tests for `ExportToClipboardUseCase` and `ExportToFileUseCase` in `src-tauri/src/application/export.rs`.
2. Implement application use cases in `src-tauri/src/application/export.rs`.
3. Implement `copy_capture_to_clipboard` and `save_capture_to_file` in `src-tauri/src/presentation/commands.rs`.
4. Implement frontend methods in `src/services/tauriBridge.ts`.

### Phase 3: Toolbar & Keyboard Integration
1. Add "Copiar (Enter)" and "Guardar (Cmd+S)" buttons to `AnnotationToolbar.ts`.
2. Wire `Enter` and `Cmd+S` keyboard listeners in `SelectionOverlay.ts`.
3. Add subtle toast confirmation.

### Phase 4: Quality Gates
1. Run `cargo test`, `vitest run`, `cargo clippy`, `cargo fmt`, `prettier`.
2. Verify all quality gates pass.