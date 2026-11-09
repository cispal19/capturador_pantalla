---
description: "Task list for feature 007-screen-recording-engine"
---

# Tasks: Motor de Grabación de Video en Región de Pantalla

**Input**: Design documents from `/specs/007-screen-recording-engine/`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: TDD approach is MANDATORY per Project Constitution (Principle I). Tests are written first and must fail before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Exact file paths are specified in every task description

---

## Phase 1: Setup & Domain Types

**Purpose**: Shared interfaces and frontend/backend types for screen recording

- [x] T001 Define TypeScript `RecordingState`, `RecordingRegion`, and `RecordingMetadata` in `src/types/recording.ts`

---

## Phase 2: Foundational (Domain Model & Mock Recorder Driver) ⚠️ Blocking

**Purpose**: Core recording data structures, validation rules, port trait, and driver mock

**⚠️ CRITICAL**: Must write tests first and ensure they fail before implementing.

- [x] T002 [P] Write unit tests for `RecordingRegion` and `RecordingState` in `src-tauri/src/domain/recording.rs`
- [x] T003 Implement `RecordingRegion` and `RecordingState` in `src-tauri/src/domain/recording.rs` and export in `src-tauri/src/domain/mod.rs`
- [x] T004 [P] Define `IScreenRecorderDriver` trait in `src-tauri/src/ports/mod.rs`
- [x] T005 [P] Write unit tests for `MockScreenRecorderDriver` in `src-tauri/src/infrastructure/mock_recorder.rs`
- [x] T006 Implement `MockScreenRecorderDriver` in `src-tauri/src/infrastructure/mock_recorder.rs` and export in `src-tauri/src/infrastructure/mod.rs`

**Checkpoint**: Foundational models and driver abstraction tested and ready.

---

## Phase 3: User Story 1 - Grabación de Video en Región Seleccionada (Priority: P1) 🎯 MVP

**Goal**: Start, stop, and persist screen recordings with even-dimension alignment and local video file output.

**Independent Test**: Select region, trigger recording, stop, verify output `.webm` file is written with valid metadata.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T007 [P] [US1] Write unit tests for `RecordingSessionManager` in `src-tauri/src/application/recording.rs`
- [x] T008 [P] [US1] Write unit tests for recording IPC commands in `src-tauri/src/presentation/commands.rs`

### Implementation for User Story 1

- [x] T009 [US1] Implement `RecordingSessionManager` in `src-tauri/src/application/recording.rs` and export in `src-tauri/src/application/mod.rs`
- [x] T010 [US1] Implement recording IPC commands in `src-tauri/src/presentation/commands.rs` and register in `src-tauri/src/lib.rs`
- [x] T011 [US1] Implement recording bridge methods in `src/services/tauriBridge.ts` with tests in `src/services/tauriBridge.test.ts`

**Checkpoint**: User Story 1 is functional: recording session lifecycle works end-to-end via IPC.

---

## Phase 4: User Stories 2 & 3 - Controles Flotantes (RecordingToolbar), Pausa/Reanudación y Límite de Tiempo (Priority: P2/P3)

**Goal**: Floating control bar with live timer, Pause/Resume toggling, and visual recording border.

**Independent Test**: Start recording, verify toolbar displays advancing timer, pause and resume, click stop.

### Tests for User Stories 2 & 3 ⚠️

- [x] T012 [P] [US2] Write unit tests for `RecordingToolbar` in `src/components/RecordingToolbar.test.ts`

### Implementation for User Stories 2 & 3

- [x] T013 [US2] Implement `RecordingToolbar` component in `src/components/RecordingToolbar.ts`
- [x] T014 [US2] Connect recording mode toggle and `RecordingToolbar` in `src/components/SelectionOverlay.ts`

**Checkpoint**: Full graphical and interactive recording experience.

---

## Phase 5: Polish & Quality Gates

**Purpose**: Documentation, quickstart validation, and quality gates

- [x] T015 [P] Update `README.md` with screen recording instructions and features
- [x] T016 Execute quickstart verification (`specs/007-screen-recording-engine/quickstart.md`)
- [x] T017 Run formatting, Clippy, and test suites (`cargo fmt -- --check && cargo clippy -- -D warnings && npm test`)
