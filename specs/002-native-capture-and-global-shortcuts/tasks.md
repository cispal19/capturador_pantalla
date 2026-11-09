---
description: "Task list for feature 002-native-capture-and-global-shortcuts"
---

# Tasks: Native Screen Capture Engine & Global Shortcuts

**Input**: Design documents from `/specs/002-native-capture-and-global-shortcuts/`
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: TDD approach is MANDATORY per Project Constitution (Principle I). Tests are written first and must fail before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Exact file paths are specified in every task description

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Dependency management and Tauri plugin configuration

- [ ] T001 Add `xcap` crate and `image` crate dependencies in `src-tauri/Cargo.toml`
- [ ] T002 [P] Add `tauri-plugin-global-shortcut` to `src-tauri/Cargo.toml` and `@tauri-apps/plugin-global-shortcut` to `package.json`
- [ ] T003 [P] Configure global shortcut plugin permissions and tray settings in `src-tauri/tauri.conf.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core models and interface traits that MUST be complete before user stories can proceed

**⚠️ CRITICAL**: No user story work can begin until this foundational phase is complete

- [ ] T004 [P] Define `DisplayInfo` Value Object in `src-tauri/src/domain/display.rs` and export in `src-tauri/src/domain/mod.rs`
- [ ] T005 [P] Define TypeScript interface `DisplayInfo` in `src/types/domain.ts`
- [ ] T006 [P] Update `IScreenCaptureDriver` and `IGlobalShortcutDriver` traits in `src-tauri/src/ports/mod.rs` per `contracts/capture-service.md`

**Checkpoint**: Foundation ready - user story implementation can now begin.

---

## Phase 3: User Story 1 - Instant Capture Trigger via Global Shortcut (Priority: P1) 🎯 MVP

**Goal**: Register global keyboard shortcuts (e.g. `Cmd+Shift+X` on macOS, `Ctrl+Shift+X` / `PrintScreen` on Windows/Linux) to trigger the screen capture session from any active window.

**Independent Test**: Register an accelerator, simulate or execute hotkey press, and verify that the capture session state transitions to `Selecting`.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [ ] T007 [P] [US1] Write unit tests for `TriggerCaptureSessionUseCase` verifying debounce logic and session state transition to `Selecting` in `src-tauri/src/application/capture.rs`
- [ ] T008 [P] [US1] Write unit tests for `MockShortcutDriver` verifying registration and unregistration in `src-tauri/src/infrastructure/mock_shortcut.rs`

### Implementation for User Story 1

- [ ] T009 [US1] Implement `TriggerCaptureSessionUseCase` in `src-tauri/src/application/capture.rs`
- [ ] T010 [US1] Implement `TauriGlobalShortcutDriver` adapter in `src-tauri/src/infrastructure/tauri_shortcut.rs`
- [ ] T011 [US1] Register global shortcut plugin and default hotkey binding in `src-tauri/src/lib.rs`
- [ ] T012 [US1] Implement frontend bridge method `registerCaptureShortcut` in `src/services/tauriBridge.ts`

**Checkpoint**: User Story 1 is functional: global shortcut triggers capture session.

---

## Phase 4: User Story 2 - High-Fidelity Multi-Monitor Screen Grabber (Priority: P2)

**Goal**: Capture full-resolution, crisp raw RGBA pixel buffers from active or multiple displays with accurate DPI scaling.

**Independent Test**: Invoke capture driver against mock or native monitors, verifying buffer dimensions, non-empty RGBA pixel matrix, and scale factor.

### Tests for User Story 2 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [ ] T013 [P] [US2] Write unit tests for `DisplayInfo` invariants (`logical_width > 0`, `scale_factor > 0.0`) in `src-tauri/src/domain/display.rs`
- [ ] T014 [P] [US2] Write unit tests for `MockScreenCaptureDriver` synthetic multi-monitor pattern generation in `src-tauri/src/infrastructure/mock_capture.rs`

### Implementation for User Story 2

- [ ] T015 [US2] Implement `MockScreenCaptureDriver` with gradient pattern generator in `src-tauri/src/infrastructure/mock_capture.rs`
- [ ] T016 [US2] Implement `XcapScreenCaptureDriver` adapter mapping physical monitors to `RawDisplayBuffer` in `src-tauri/src/infrastructure/xcap_capture.rs`
- [ ] T017 [US2] Implement Tauri IPC commands `get_connected_monitors` and `trigger_screen_capture` in `src-tauri/src/presentation/commands.rs`
- [ ] T018 [US2] Implement frontend bridge methods `getConnectedMonitors` and `triggerScreenCapture` in `src/services/tauriBridge.ts`
- [ ] T019 [P] [US2] Write frontend unit tests for monitor discovery in `src/services/tauriBridge.test.ts`

**Checkpoint**: User Story 2 is functional: displays can be enumerated and captured into raw memory buffers.

---

## Phase 5: User Story 3 - System Tray Quick Actions & OS Permissions Guidance (Priority: P3)

**Goal**: Provide a persistent system tray icon with quick actions and friendly instructions if OS screen recording permissions are missing.

**Independent Test**: Verify tray menu options ("Tomar Captura", "Salir") and permission check handler without crashing when permission is missing.

### Tests for User Story 3 ⚠️

- [ ] T020 [P] [US3] Write unit tests for permission error handling and validation in `src-tauri/src/infrastructure/permissions.rs`

### Implementation for User Story 3

- [ ] T021 [US3] Implement System Tray menu builder and click event dispatchers in `src-tauri/src/presentation/tray.rs`
- [ ] T022 [US3] Implement OS screen recording permission detection in `src-tauri/src/infrastructure/permissions.rs`
- [ ] T023 [US3] Mount System Tray in application runtime in `src-tauri/src/lib.rs`

**Checkpoint**: All user stories complete and functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, quickstart verification, and static analysis

- [ ] T024 [P] Update `README.md` with shortcut configuration and system tray documentation
- [ ] T025 Execute end-to-end quickstart validation following `specs/002-native-capture-and-global-shortcuts/quickstart.md`
- [ ] T026 Run formatting, Clippy, and test suites (`cargo fmt -- --check && cargo clippy -- -D warnings && npm test`)

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup (T001-T003)
       │
       ▼
Phase 2: Foundational (T004-T006) ──► (BLOCKS US1, US2, US3)
       │
       ├─────────────────────────┐
       ▼                         ▼
Phase 3: US1 (P1) Hotkeys    Phase 4: US2 (P2) Screen Grabber
       │                         │
       └───────────┬─────────────┘
                   │
                   ▼
       Phase 5: US3 (P3) Tray & Permissions
                   │
                   ▼
       Phase 6: Polish (T024-T026)
```
