---
description: "Task list for feature 001-scaffolding-clean-architecture"
---

# Tasks: Project Scaffolding & Clean Architecture Foundation

**Input**: Design documents from `/specs/001-scaffolding-clean-architecture/`
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: TDD approach is MANDATORY per Project Constitution (Principle I). Tests are written first and must fail before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Exact file paths are specified in every task description

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, dependency management, and configuration

- [x] T001 Initialize root package with Node.js dependencies, build scripts (`test:backend`, `test:frontend`, `lint`, `format:check`), and Tauri CLI in `package.json`
- [x] T002 [P] Configure TypeScript compiler options, strict type checking, and path aliases in `tsconfig.json`
- [x] T003 [P] Configure Vite build tooling with development port and Tauri optimizations in `vite.config.ts`
- [x] T004 [P] Initialize Rust package with dependencies (`tauri`, `serde`, `serde_json`, `thiserror`) and dev-dependencies in `src-tauri/Cargo.toml`
- [x] T005 [P] Configure Tauri 2.0 application metadata, identifier `com.capturador.app`, security permissions, and background window settings in `src-tauri/tauri.conf.json`
- [x] T006 Setup Tauri build script in `src-tauri/build.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Clean Architecture core boundaries and ports that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this foundational phase is complete

- [x] T007 Create Clean Architecture directory layout in `src-tauri/src/` (`domain/`, `application/`, `ports/`, `infrastructure/`, `presentation/`) and `src/` (`components/`, `services/`, `types/`)
- [x] T008 [P] Define domain error enumerations in `src-tauri/src/domain/error.rs` and export module in `src-tauri/src/domain/mod.rs`
- [x] T009 [P] Define boundary port traits `IScreenCaptureDriver`, `IClipboardDriver`, `IFileSystemDriver`, `IGlobalShortcutDriver` in `src-tauri/src/ports/mod.rs` per `contracts/ports.md`
- [x] T010 [P] Define TypeScript interfaces mirroring Rust domain contracts (`Coordinates`, `Dimensions`, `BoundingBox`, `AppInfo`) in `src/types/domain.ts`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel or sequentially.

---

## Phase 3: User Story 1 - Application Lifecycle & Local Development Bootstrap (Priority: P1) 🎯 MVP

**Goal**: Launch and run the application in a local development environment with a single command, verifying that the core runtime initializes cleanly, resides in the background without crashing, and terminates gracefully.

**Independent Test**: Execute `npm run tauri dev` or headless binary test, verifying the process boots, initializes the background runtime with exit code 0 on termination.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T011 [P] [US1] Write lifecycle unit test for background application runtime initialization and clean shutdown in `src-tauri/src/lib.rs`

### Implementation for User Story 1

- [x] T012 [P] [US1] Implement minimal frontend HTML shell and viewport container in `index.html` and `src/main.ts`
- [x] T013 [P] [US1] Implement root desktop presentation component with application status and Local-First guarantee banner in `src/App.tsx`
- [x] T014 [US1] Implement Tauri IPC command `get_app_info` in `src-tauri/src/presentation/commands.rs` returning app name, version, and `local_first_verified: true` per `contracts/ipc-commands.md`
- [x] T015 [US1] Configure Tauri desktop entrypoint, plugin registration, and command routing in `src-tauri/src/lib.rs` and `src-tauri/src/main.rs`
- [x] T016 [US1] Implement typed IPC bridge client in `src/services/tauriBridge.ts` invoking `get_app_info` command

**Checkpoint**: User Story 1 should be fully functional and testable independently (MVP bootstrap complete).

---

## Phase 4: User Story 2 - Automated Test Harness & Quality Gates (Priority: P2)

**Goal**: Enforce automated testing, static analysis (Clippy, ESLint), formatting, and security audits across platforms from Day 1.

**Independent Test**: Run `npm run test:backend`, `npm run test:frontend`, and `npm run lint` locally and in CI, verifying that compliant code passes 100% and violations fail with non-zero exit code.

### Tests for User Story 2 ⚠️

- [x] T017 [P] [US2] Configure Vitest test runner configuration for frontend tests in `vitest.config.ts`
- [x] T018 [P] [US2] Write unit test for `tauriBridge` client verifying IPC invocation and fallback handling in `src/services/tauriBridge.test.ts`

### Implementation for User Story 2

- [x] T019 [P] [US2] Configure ESLint and Prettier rules for strict TypeScript and code formatting in `eslint.config.js` and `.prettierrc`
- [x] T020 [P] [US2] Configure Rust code formatting standards in `src-tauri/rustfmt.toml`
- [x] T021 [US2] Create cross-platform CI matrix workflow (macOS, Windows, Ubuntu) in `.github/workflows/ci.yml` running `cargo test`, `cargo clippy`, `cargo audit`, and frontend lint/tests

**Checkpoint**: User Story 2 is functional; automated test runner and quality gates protect code integrity locally and in CI.

---

## Phase 5: User Story 3 - Architectural Boundary & Domain Isolation (Priority: P3)

**Goal**: Decouple core domain logic (geometry, bounding box normalization, clipping, and pixel buffer representation) from OS APIs and UI, enabling 100% headless test execution.

**Independent Test**: Run `cargo test` in a headless terminal without display server and verify all coordinate transformations and invariants evaluate in < 1 second.

### Tests for User Story 3 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T022 [P] [US3] Write unit tests for `DisplayCoordinates` (`x, y` must be finite) and `Dimensions` (`width >= 0.0`, `height >= 0.0`) in `src-tauri/src/domain/coordinates.rs` and `src-tauri/src/domain/dimensions.rs`
- [x] T023 [P] [US3] Write unit tests for `BoundingBox` normalization (inverted drags $X_1 > X_2, Y_1 > Y_2$), boundary clamping, and physical pixel scaling in `src-tauri/src/domain/bounding_box.rs`
- [x] T024 [P] [US3] Write unit tests for `RawDisplayBuffer` memory layout invariants (`stride >= width * 4`) and `CaptureSessionState` transitions in `src-tauri/src/domain/buffer.rs` and `src-tauri/src/application/session.rs`

### Implementation for User Story 3

- [x] T025 [US3] Implement `DisplayCoordinates` and `Dimensions` Value Objects with validation methods in `src-tauri/src/domain/coordinates.rs` and `src-tauri/src/domain/dimensions.rs`
- [x] T026 [US3] Implement `BoundingBox` Value Object with `from_points()`, `clamp()`, and `to_physical_pixels()` in `src-tauri/src/domain/bounding_box.rs`
- [x] T027 [US3] Implement `RawDisplayBuffer` entity and `CaptureSessionState` state machine in `src-tauri/src/domain/buffer.rs` and `src-tauri/src/application/session.rs`
- [x] T028 [US3] Implement Tauri IPC commands `normalize_bounding_box` and `get_session_state` in `src-tauri/src/presentation/commands.rs` per `contracts/ipc-commands.md`

**Checkpoint**: All three user stories are complete, independently functional, and verified by isolated tests.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, runnable verification, and security audit hardening

- [x] T029 [P] Document developer setup, architecture diagram, and quickstart commands in `README.md`
- [x] T030 Execute end-to-end quickstart validation following `specs/001-scaffolding-clean-architecture/quickstart.md`
- [x] T031 Run security audit (`cargo audit` and `npm audit`) verifying 0 critical or high CVEs

---

## Dependencies & Execution Order

### Phase Dependencies

```
┌──────────────────────────────────────┐
│        Phase 1: Setup (T001-T006)    │
└──────────────────┬───────────────────┘
                   │
                   ▼
┌──────────────────────────────────────┐
│     Phase 2: Foundational (T007-T010)│  (BLOCKS ALL USER STORIES)
└──────────┬───────────────┬───────────┘
           │               │
           ▼               ▼
┌────────────────────┐   ┌────────────────────┐
│ Phase 3: US1 (P1)  │   │ Phase 4: US2 (P2)  │  (Can proceed in parallel)
│  MVP Bootstrap     │   │  CI & Test Gates   │
└──────────┬─────────┘   └─────────┬──────────┘
           │                       │
           └───────────┬───────────┘
                       │
                       ▼
┌──────────────────────────────────────┐
│ Phase 5: US3 (P3) Domain Core        │
└──────────────────┬───────────────────┘
                   │
                   ▼
┌──────────────────────────────────────┐
│   Phase 6: Polish (T029-T031)        │
└──────────────────────────────────────┘
```

### User Story Dependencies

- **User Story 1 (P1)**: Depends only on Foundational (Phase 2). Delivers viable MVP bootstrap.
- **User Story 2 (P2)**: Depends only on Foundational (Phase 2). Can run in parallel with US1.
- **User Story 3 (P3)**: Depends on Foundational (Phase 2). Implements domain core logic and connects with US1 presentation commands.

---

## Parallel Execution Examples

### Parallel Example: Phase 1 (Setup)
```bash
# Launch independent config files in parallel:
Task: "T002 [P] Configure TypeScript compiler in tsconfig.json"
Task: "T003 [P] Configure Vite build tooling in vite.config.ts"
Task: "T004 [P] Initialize Rust package in src-tauri/Cargo.toml"
Task: "T005 [P] Configure Tauri 2.0 application settings in src-tauri/tauri.conf.json"
```

### Parallel Example: User Story 3 (Domain Isolation)
```bash
# Launch independent domain tests in parallel (TDD Red Phase):
Task: "T022 [P] [US3] Write unit tests for DisplayCoordinates and Dimensions"
Task: "T023 [P] [US3] Write unit tests for BoundingBox normalization"
Task: "T024 [P] [US3] Write unit tests for RawDisplayBuffer and CaptureSessionState"
```

---

## Implementation Strategy

### MVP First (User Story 1 Focus)
1. Complete Phase 1 (Setup) and Phase 2 (Foundational).
2. Implement Phase 3 (User Story 1).
3. **Validate MVP**: Confirm application launches, stays in background, and responds to `get_app_info`.

### Incremental Quality & Domain Delivery
1. Implement Phase 4 (User Story 2): Enable CI matrix and Vitest.
2. Implement Phase 5 (User Story 3): Build isolated domain models with 100% test pass rate in headless mode.
3. Complete Phase 6 (Polish): Execute quickstart validation and security audits.
