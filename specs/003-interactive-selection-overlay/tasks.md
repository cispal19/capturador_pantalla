---
description: "Task list for feature 003-interactive-selection-overlay"
---

# Tasks: Interactive Selection Overlay & Resizing

**Input**: Design documents from `/specs/003-interactive-selection-overlay/`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: TDD approach is MANDATORY per Project Constitution (Principle I). Unit tests are written first and must fail before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Exact file paths are specified in every task description

---

## Phase 1: Setup (Shared Infrastructure & Types)

**Purpose**: Shared types, configuration, and interface definitions

- [ ] T001 [P] Define `ResizeHandle` and `HandlePosition` types in `src/types/selection.ts`
- [ ] T002 [P] Configure transparent window settings in `src-tauri/tauri.conf.json` for full-screen overlay

---

## Phase 2: Foundational (Geometric Domain Models) ⚠️ Blocking

**Purpose**: Core mathematical models in pure Rust and pure TypeScript that MUST be complete before user stories proceed

**⚠️ CRITICAL**: TDD required. Write tests first, ensure they fail, then implement.

- [ ] T003 [P] Write unit tests for `ResizeHandle` and handle anchor calculations in `src-tauri/src/domain/handle.rs`
- [ ] T004 [P] Implement `ResizeHandle` enum and cursor mappings in `src-tauri/src/domain/handle.rs` and export in `src-tauri/src/domain/mod.rs`
- [ ] T005 [P] Write unit tests for `BoundingBox` geometry methods (`handle_anchors`, `hit_test_handle`, `resize_by_handle`, `translate`) in `src-tauri/src/domain/bounding_box.rs`
- [ ] T006 Implement geometry extension methods (`handle_anchors`, `hit_test_handle`, `resize_by_handle`, `translate`) in `src-tauri/src/domain/bounding_box.rs`
- [ ] T007 [P] Write frontend unit tests for geometry and normalization math in `src/domain/geometry.test.ts`
- [ ] T008 [P] Implement frontend pure geometry library (`normalizeSelection`, `clampRect`, `getHandleAnchors`, `hitTestHandle`) in `src/domain/geometry.ts`

**Checkpoint**: Foundational geometric domains ready and tested in both Rust and TypeScript.

---

## Phase 3: User Story 1 - Demarcación Inicial Rectangular con Arrastre (Priority: P1) 🎯 MVP

**Goal**: Full-screen 40% dimmed canvas overlay allowing smooth drag-to-select in any direction with real-time dimensions indicator (`W x H px`) at 60 FPS.

**Independent Test**: Mount the overlay, simulate mouse drag from `(100, 100)` to `(500, 400)`, verify normalized box `400 x 300 px`, dimming cutout, and dimensions badge.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [ ] T009 [P] [US1] Write unit tests for `SelectionOverlay` initial drawing and inverted drag normalization in `src/components/SelectionOverlay.test.ts`
- [ ] T010 [P] [US1] Write unit tests for `commit_selection_region` IPC command in `src-tauri/src/presentation/commands.rs`

### Implementation for User Story 1

- [ ] T011 [US1] Implement `commit_selection_region` IPC command in `src-tauri/src/presentation/commands.rs` and register in `src-tauri/src/lib.rs`
- [ ] T012 [US1] Implement `SelectionOverlay` HTML5 Canvas 2D component with 60 FPS `requestAnimationFrame` drawing and inverted drag support in `src/components/SelectionOverlay.ts`
- [ ] T013 [US1] Implement frontend bridge methods `commitSelectionRegion` in `src/services/tauriBridge.ts` with tests in `src/services/tauriBridge.test.ts`
- [ ] T014 [US1] Mount `SelectionOverlay` in `src/main.ts` on session state transition to `Selecting`

**Checkpoint**: User Story 1 is functional: users can drag to select an area with live dimensions at 60 FPS.

---

## Phase 4: User Story 2 - Ajuste Fino con 8 Manijas y Desplazamiento (Priority: P2)

**Goal**: Enable fine-tuning of selection via 8 interactive resize handles and translation by dragging the interior of the box.

**Independent Test**: Given a selection, drag the `SE` handle to resize, and drag the body to translate. Verify coordinates update smoothly and respect screen bounds.

### Tests for User Story 2 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [ ] T015 [P] [US2] Write unit tests for handle drag resizing and body translation in `src/components/SelectionOverlay.test.ts`
- [ ] T016 [P] [US2] Write unit tests for boundary clamping during resize/move in `src/domain/geometry.test.ts`

### Implementation for User Story 2

- [ ] T017 [US2] Implement 8-handle visual rendering (white squares with corporate blue borders) and dynamic cursor styling in `src/components/SelectionOverlay.ts`
- [ ] T018 [US2] Implement handle drag interaction logic (`mode: 'resizing'`) in `src/components/SelectionOverlay.ts`
- [ ] T019 [US2] Implement body translation drag interaction logic (`mode: 'moving'`) in `src/components/SelectionOverlay.ts`

**Checkpoint**: User Stories 1 and 2 are functional: full selection, resizing, and repositioning work seamlessly.

---

## Phase 5: User Story 3 - Cancelación Inmediata con Escape (Priority: P3)

**Goal**: Provide instantaneous, clean session abort when the user presses `Escape`, restoring state to `Idle`.

**Independent Test**: While overlay is active or during a drag, press `Esc`. Overlay hides immediately and session returns to `Idle`.

### Tests for User Story 3 ⚠️

- [ ] T020 [P] [US3] Write unit tests for `cancel_capture_session` IPC command and `cancelCaptureSession` bridge in `src-tauri/src/presentation/commands.rs` and `src/services/tauriBridge.test.ts`
- [ ] T021 [P] [US3] Write unit tests for `Escape` keyboard event handler in `src/components/SelectionOverlay.test.ts`

### Implementation for User Story 3

- [ ] T022 [US3] Implement `cancel_capture_session` IPC command in `src-tauri/src/presentation/commands.rs`
- [ ] T023 [US3] Implement `cancelCaptureSession` method in `src/services/tauriBridge.ts`
- [ ] T024 [US3] Wire window keydown listener for `Escape` in `src/components/SelectionOverlay.ts` dispatching `cancelCaptureSession`

**Checkpoint**: All user stories complete and functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, quickstart validation, and static analysis

- [ ] T025 [P] Update `README.md` and `docs/PRD.md` with interactive selection overlay capabilities
- [ ] T026 Execute end-to-end quickstart validation per `specs/003-interactive-selection-overlay/quickstart.md`
- [ ] T027 Run formatting, Clippy, and test suites (`cargo fmt -- --check && cargo clippy -- -D warnings && npm test`)

---

## Dependencies & Execution Order

```
Phase 1: Setup (T001-T002)
       │
       ▼
Phase 2: Foundational Geometry (T003-T008) ──► (BLOCKS US1, US2, US3)
       │
       ├─────────────────────────┐
       ▼                         ▼
Phase 3: US1 (P1) Selection   Phase 4: US2 (P2) 8-Handles & Move
       │                         │
       └───────────┬─────────────┘
                   │
                   ▼
       Phase 5: US3 (P3) Escape Cancellation
                   │
                   ▼
       Phase 6: Polish & Quality Gates (T025-T027)
```