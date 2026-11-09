---
description: "Task list for feature 008-gif-recording-and-optimization"
---

# Tasks: Exportación y Optimización de GIFs Animados

**Input**: Design documents from `/specs/008-gif-recording-and-optimization/`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: TDD approach is MANDATORY per Project Constitution (Principle I). Tests are written first and must fail before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Exact file paths are specified in every task description

---

## Phase 1: Setup & Dependencies

**Purpose**: Add GIF codec feature in Rust backend and TypeScript interfaces

- [ ] T001 Add `gif` feature to `image` crate dependency in `src-tauri/Cargo.toml`
- [ ] T002 Define TypeScript `GifExportConfig` and `GifExportResult` in `src/types/gif.ts`

---

## Phase 2: Foundational (Domain Model & Naming) ⚠️ Blocking

**Purpose**: Core configuration value object, validation rules, and file naming

**⚠️ CRITICAL**: Must write tests first and ensure they fail before implementing.

- [ ] T003 [P] Write unit tests for `GifExportConfig` and `GifExportResult` in `src-tauri/src/domain/gif.rs`
- [ ] T004 Implement `GifExportConfig` and `GifExportResult` in `src-tauri/src/domain/gif.rs` and export in `src-tauri/src/domain/mod.rs`
- [ ] T005 [P] Implement `generate_animation_name` in `src-tauri/src/domain/naming.rs` with unit tests

**Checkpoint**: Foundational model and naming helpers tested and ready.

---

## Phase 3: User Stories 1 & 2 - Codificación y Optimización de GIFs Animados (Priority: P1/P2) 🎯 MVP

**Goal**: Ingest multi-frame RGBA buffers, encode valid looping GIF89a file, and save locally.

**Independent Test**: Provide synthetic sequence of 3 RGBA frames, run export use case, verify generated file begins with `GIF89a` magic bytes and contains all frames.

### Tests for User Story 1 & 2 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [ ] T006 [P] [US1] Write unit tests for `ExportRecordingToGifUseCase` in `src-tauri/src/application/gif.rs`
- [ ] T007 [P] [US1] Write unit tests for `export_frames_to_gif` IPC command in `src-tauri/src/presentation/commands.rs`

### Implementation for User Story 1 & 2

- [ ] T008 [US1] Implement `ExportRecordingToGifUseCase` in `src-tauri/src/application/gif.rs` and export in `src-tauri/src/application/mod.rs`
- [ ] T009 [US1] Implement `export_frames_to_gif` IPC command in `src-tauri/src/presentation/commands.rs` and register in `src-tauri/src/lib.rs`
- [ ] T010 [US1] Implement `exportFramesToGif` bridge method in `src/services/tauriBridge.ts` with tests in `src/services/tauriBridge.test.ts`

**Checkpoint**: User Stories 1 & 2 functional: animated GIFs are encoded and persisted via IPC.

---

## Phase 4: User Story 3 - Interfaz de Usuario y Disparador de GIF (Priority: P3)

**Goal**: Add "🎞️ GIF" button to toolbar and connect overlay export flow.

**Independent Test**: Select area, click "🎞️ GIF", verify export method is invoked and toast notification confirms path.

### Tests for User Story 3 ⚠️

- [ ] T011 [P] [US3] Write unit tests for "🎞️ GIF" button in `src/components/AnnotationToolbar.test.ts`

### Implementation for User Story 3

- [ ] T012 [US3] Add "🎞️ GIF" button and callback in `src/components/AnnotationToolbar.ts`
- [ ] T013 [US3] Connect `exportToGif` in `src/components/SelectionOverlay.ts` with tests in `src/components/SelectionOverlay.test.ts`

**Checkpoint**: Complete end-to-end user experience for animated GIF creation.

---

## Phase 5: Polish & Quality Gates

**Purpose**: Documentation, quickstart validation, and quality gates

- [ ] T014 [P] Update `README.md` with GIF export instructions and full roadmap completion notice
- [ ] T015 Execute quickstart verification (`specs/008-gif-recording-and-optimization/quickstart.md`)
- [ ] T016 Run formatting, Clippy, and test suites (`cargo fmt -- --check && cargo clippy -- -D warnings && npm test`)
