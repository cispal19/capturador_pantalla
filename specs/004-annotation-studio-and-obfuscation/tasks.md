---
description: "Task list for feature 004-annotation-studio-and-obfuscation"
---

# Tasks: Annotation Studio & Irreversible Obfuscation Engine

**Input**: Design documents from `/specs/004-annotation-studio-and-obfuscation/`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: TDD approach is MANDATORY per Project Constitution (Principle I). Tests are written first and must fail before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Exact file paths are specified in every task description

---

## Phase 1: Setup (Shared Infrastructure & Types)

**Purpose**: Define annotation and tool types in frontend and backend

- [x] T001 [P] Define `AnnotationItem`, `ActiveTool`, `CorporateColor`, and `StrokeThickness` in `src/types/annotations.ts`
- [x] T002 [P] Define `AnnotationTool` enum and `Annotation` discriminated enum in `src-tauri/src/domain/annotation.rs` and export in `src-tauri/src/domain/mod.rs`

---

## Phase 2: Foundational (Core Mathematical Models) ⚠️ Blocking

**Purpose**: Core mathematical models for irreversible obfuscation and undo/redo history

**⚠️ CRITICAL**: Must write tests first and ensure they fail before implementing.

- [x] T003 [P] Write unit tests for `ObfuscationKernel` (block-averaging, irreversibility, boundary safety) in `src-tauri/src/domain/obfuscation.rs`
- [x] T004 Implement `ObfuscationKernel::apply_pixelate` in `src-tauri/src/domain/obfuscation.rs` and export in `src-tauri/src/domain/mod.rs`
- [x] T005 [P] Write unit tests for `UndoRedoHistory` in `src-tauri/src/domain/annotation.rs`
- [x] T006 Implement `UndoRedoHistory<T>` container in `src-tauri/src/domain/annotation.rs`
- [x] T007 [P] Write frontend unit tests for `UndoRedoStack` in `src/domain/annotations.test.ts`
- [x] T008 [P] Implement `UndoRedoStack<T>` in `src/domain/annotations.ts`

**Checkpoint**: Foundational models and mathematical kernels verified and tested.

---

## Phase 3: User Story 1 - Ofuscación Destructiva e Irreversible (Priority: P1) 🎯 MVP

**Goal**: Apply irreversible block-averaging pixelation to sensitive areas (passwords, tokens, PII) on screen.

**Independent Test**: Draw an obfuscation rectangle over high-contrast pixel data, verify the resulting block pixels are flattened to arithmetic mean RGB values.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T009 [P] [US1] Write unit tests for frontend canvas obfuscator in `src/domain/obfuscation.test.ts`
- [x] T010 [P] [US1] Write unit tests for `apply_obfuscation` IPC command in `src-tauri/src/presentation/commands.rs`

### Implementation for User Story 1

- [x] T011 [US1] Implement `apply_obfuscation` IPC command in `src-tauri/src/presentation/commands.rs` and register in `src-tauri/src/lib.rs`
- [x] T012 [US1] Implement `applyPixelationToCanvas` in `src/domain/obfuscation.ts`
- [x] T013 [US1] Implement frontend bridge method `applyObfuscation` in `src/services/tauriBridge.ts`
- [x] T014 [US1] Wire obfuscation drawing tool in `src/components/SelectionOverlay.ts`

**Checkpoint**: User Story 1 is functional: sensitive areas can be destroyed irreversibly with pixelation.

---

## Phase 4: User Story 2 - Estudio de Herramientas Vectoriales y Paleta Corporativa (Priority: P2)

**Goal**: Provide a floating toolbar with shapes (rectangle, ellipse, arrow, line, freehand, text, highlighter) and color/thickness selectors.

**Independent Test**: Select "Arrow" in red, drag across selection, verify crisp arrow vector render. Select "Highlighter", drag over text, verify 40% translucent yellow stroke.

### Tests for User Story 2 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T015 [P] [US2] Write unit tests for `AnnotationToolbar` positioning and tool/color selection in `src/components/AnnotationToolbar.test.ts`
- [x] T016 [P] [US2] Write unit tests for vector shape generation helpers in `src/domain/annotations.test.ts`

### Implementation for User Story 2

- [x] T017 [US2] Implement vector shape render helpers (rect, ellipse, arrow, line, freehand, text, highlighter) in `src/domain/annotations.ts`
- [x] T018 [US2] Implement `AnnotationToolbar` component with tool/color/thickness palette in `src/components/AnnotationToolbar.ts`
- [x] T019 [US2] Integrate `AnnotationToolbar` and active drawing into `src/components/SelectionOverlay.ts`

**Checkpoint**: User Stories 1 and 2 are functional: vector tools and floating palette work seamlessly.

---

## Phase 5: User Story 3 - Historial Completo de Deshacer y Rehacer (Priority: P3)

**Goal**: Full undo/redo capability (`Ctrl+Z` / `Cmd+Z`, `Ctrl+Y` / `Cmd+Shift+Z`) for annotations and obfuscations.

**Independent Test**: Draw a rectangle, then text, then blur. Undo twice: text and blur disappear. Redo once: text reappears.

### Tests for User Story 3 ⚠️

- [x] T020 [P] [US3] Write unit tests for keyboard shortcuts (`Cmd+Z`, `Cmd+Shift+Z`) and undo/redo dispatching in `src/components/SelectionOverlay.test.ts`

### Implementation for User Story 3

- [x] T021 [US3] Wire `UndoRedoStack` into `SelectionOverlay.ts` for all annotations and obfuscations
- [x] T022 [US3] Wire keyboard listeners for `Ctrl+Z`, `Cmd+Z`, `Ctrl+Y`, `Cmd+Shift+Z` and toolbar buttons in `src/components/SelectionOverlay.ts`
- [x] T023 [US3] Implement `sync_annotations` IPC command in `src-tauri/src/presentation/commands.rs` and `syncAnnotations` bridge in `src/services/tauriBridge.ts`

**Checkpoint**: All user stories complete and functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, quickstart validation, and quality gates

- [x] T024 [P] Update `README.md` with annotation studio and obfuscation instructions
- [x] T025 Execute quickstart validation following `specs/004-annotation-studio-and-obfuscation/quickstart.md`
- [x] T026 Run formatting, Clippy, and test suites (`cargo fmt -- --check && cargo clippy -- -D warnings && npm test`)

---

## Dependencies & Execution Order

```
Phase 1: Setup (T001-T002)
       │
       ▼
Phase 2: Foundational (T003-T008) ──► (BLOCKS US1, US2, US3)
       │
       ├─────────────────────────┐
       ▼                         ▼
Phase 3: US1 (P1) Obfuscation  Phase 4: US2 (P2) Vector Tools
       │                         │
       └───────────┬─────────────┘
                   │
                   ▼
       Phase 5: US3 (P3) Undo / Redo
                   │
                   ▼
       Phase 6: Polish & Quality Gates (T024-T026)
```