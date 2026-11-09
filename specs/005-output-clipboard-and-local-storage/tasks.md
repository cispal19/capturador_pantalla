---
description: "Task list for feature 005-output-clipboard-and-local-storage"
---

# Tasks: Output Pipeline, Clipboard & Local Storage

**Input**: Design documents from `/specs/005-output-clipboard-and-local-storage/`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: TDD approach is MANDATORY per Project Constitution (Principle I). Tests are written first and must fail before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Exact file paths are specified in every task description

---

## Phase 1: Setup (Shared Infrastructure & Dependencies)

**Purpose**: Add `arboard` crate dependency and storage formats

- [x] T001 Add `arboard = "3"` dependency and JPEG feature to `src-tauri/Cargo.toml`

---

## Phase 2: Foundational (Domain Helpers & Adapters) ⚠️ Blocking

**Purpose**: Core naming generator, clipboard and storage drivers

**⚠️ CRITICAL**: Must write tests first and ensure they fail before implementing.

- [x] T002 [P] Write unit tests for `FileNameGenerator` in `src-tauri/src/domain/naming.rs`
- [x] T003 Implement `FileNameGenerator` in `src-tauri/src/domain/naming.rs` and export in `src-tauri/src/domain/mod.rs`
- [x] T004 [P] Write unit tests for `MockClipboardDriver` in `src-tauri/src/infrastructure/clipboard.rs`
- [x] T005 Implement `ArboardClipboardDriver` and `MockClipboardDriver` in `src-tauri/src/infrastructure/clipboard.rs` and export in `src-tauri/src/infrastructure/mod.rs`
- [x] T006 [P] Write unit tests for `FileSystemStorageDriver` in `src-tauri/src/infrastructure/storage.rs`
- [x] T007 Implement `FileSystemStorageDriver` encoding PNG/JPEG buffers in `src-tauri/src/infrastructure/storage.rs` and export in `src-tauri/src/infrastructure/mod.rs`

**Checkpoint**: Foundational drivers and models tested and ready.

---

## Phase 3: User Story 1 - Copiado Inmediato al Portapapeles (Priority: P1) 🎯 MVP

**Goal**: Transfer composed screenshot directly to system clipboard as `image/png` on `Enter` key or "Copiar" button, then close overlay.

**Independent Test**: Mount overlay with selection, press `Enter`, verify `copy_capture_to_clipboard` is called with RGBA data, clipboard contains image, and overlay closes.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T008 [P] [US1] Write unit tests for `ExportToClipboardUseCase` in `src-tauri/src/application/export.rs`
- [x] T009 [P] [US1] Write unit tests for `copy_capture_to_clipboard` IPC command in `src-tauri/src/presentation/commands.rs`

### Implementation for User Story 1

- [x] T010 [US1] Implement `ExportToClipboardUseCase` in `src-tauri/src/application/export.rs`
- [x] T011 [US1] Implement `copy_capture_to_clipboard` IPC command in `src-tauri/src/presentation/commands.rs` and register in `src-tauri/src/lib.rs`
- [x] T012 [US1] Implement `copyCaptureToClipboard` method in `src/services/tauriBridge.ts` with tests in `src/services/tauriBridge.test.ts`
- [x] T013 [US1] Add "Copiar (Enter)" action button to `AnnotationToolbar.ts` and handle `Enter` key in `SelectionOverlay.ts`

**Checkpoint**: User Story 1 is functional: users can press `Enter` or click "Copiar" to transfer screenshot to clipboard.

---

## Phase 4: User Story 2 - Guardado en Disco Local (Priority: P2)

**Goal**: Save screenshot to local disk (PNG/JPEG) with ISO timestamp naming pattern on `Ctrl+S` / `Cmd+S` or "Guardar" button.

**Independent Test**: Press `Cmd+S`, verify `save_capture_to_file` executes, writes image file to disk, and returns file path.

### Tests for User Story 2 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T014 [P] [US2] Write unit tests for `ExportToFileUseCase` in `src-tauri/src/application/export.rs`
- [x] T015 [P] [US2] Write unit tests for `save_capture_to_file` IPC command in `src-tauri/src/presentation/commands.rs`

### Implementation for User Story 2

- [x] T016 [US2] Implement `ExportToFileUseCase` in `src-tauri/src/application/export.rs`
- [x] T017 [US2] Implement `save_capture_to_file` IPC command in `src-tauri/src/presentation/commands.rs` and register in `src-tauri/src/lib.rs`
- [x] T018 [US2] Implement `saveCaptureToFile` in `src/services/tauriBridge.ts` with tests in `src/services/tauriBridge.test.ts`
- [x] T019 [US2] Add "Guardar (Cmd+S)" action button to `AnnotationToolbar.ts` and handle `Cmd+S` / `Ctrl+S` in `SelectionOverlay.ts`

**Checkpoint**: User Stories 1 and 2 are functional: clipboard copy and disk saving work cleanly.

---

## Phase 5: User Story 3 - Notificación Visual Sutil de Confirmación (Priority: P3)

**Goal**: Show subtle toast notification confirming screenshot was copied or saved.

**Independent Test**: Trigger copy/save, verify toast notification appears and dismisses smoothly.

### Tests for User Story 3 ⚠️

- [x] T020 [P] [US3] Write unit tests for toast notification helper in `src/components/Toast.test.ts`

### Implementation for User Story 3

- [x] T021 [US3] Implement `showToast(message)` helper in `src/components/Toast.ts`
- [x] T022 [US3] Wire toast notification upon copy or save in `src/components/SelectionOverlay.ts`

**Checkpoint**: All user stories complete and functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, quickstart validation, and quality gates

- [x] T023 [P] Update `README.md` with clipboard copy and local save instructions
- [x] T024 Execute quickstart validation following `specs/005-output-clipboard-and-local-storage/quickstart.md`
- [x] T025 Run formatting, Clippy, and test suites (`cargo fmt -- --check && cargo clippy -- -D warnings && npm test`)

---

## Dependencies & Execution Order

```
Phase 1: Setup (T001)
       │
       ▼
Phase 2: Foundational (T002-T007) ──► (BLOCKS US1, US2, US3)
       │
       ├─────────────────────────┐
       ▼                         ▼
Phase 3: US1 (P1) Clipboard   Phase 4: US2 (P2) Local File
       │                         │
       └───────────┬─────────────┘
                   │
                   ▼
       Phase 5: US3 (P3) Toast Notification
                   │
                   ▼
       Phase 6: Polish & Quality Gates (T023-T025)
```