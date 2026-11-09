---
description: "Task list for feature 006-settings-and-user-preferences"
---

# Tasks: Panel de Configuración y Preferencias de Usuario

**Input**: Design documents from `/specs/006-settings-and-user-preferences/`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: TDD approach is MANDATORY per Project Constitution (Principle I). Tests are written first and must fail before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Exact file paths are specified in every task description

---

## Phase 1: Setup & Domain Types

**Purpose**: Shared interfaces and frontend/backend types for configuration

- [x] T001 Define TypeScript `AppPreferences` interface and defaults in `src/types/preferences.ts`

---

## Phase 2: Foundational (Domain Model & Config Storage Driver) ⚠️ Blocking

**Purpose**: Core preferences data structure, validation, and JSON store driver

**⚠️ CRITICAL**: Must write tests first and ensure they fail before implementing.

- [x] T002 [P] Write unit tests for `AppPreferences` domain struct, validation, and serialization in `src-tauri/src/domain/preferences.rs`
- [x] T003 Implement `AppPreferences` in `src-tauri/src/domain/preferences.rs` and export in `src-tauri/src/domain/mod.rs`
- [x] T004 [P] Write unit tests for `JsonConfigStoreDriver` in `src-tauri/src/infrastructure/config_store.rs`
- [x] T005 Implement `JsonConfigStoreDriver` in `src-tauri/src/infrastructure/config_store.rs` and export in `src-tauri/src/infrastructure/mod.rs`

**Checkpoint**: Foundational model and JSON store driver tested and ready.

---

## Phase 3: User Story 1 - Persistencia y Gestión del Esquema de Preferencias Locales (Priority: P1) 🎯 MVP

**Goal**: Load, validate, update, and reset `AppPreferences` atomically using `config.json` in user space without administrator privileges.

**Independent Test**: Read preferences, update fields, save, reload from disk and verify values persist.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T006 [P] [US1] Write unit tests for `ManagePreferencesUseCase` in `src-tauri/src/application/preferences.rs`
- [x] T007 [P] [US1] Write unit tests for IPC commands (`get_preferences`, `update_preferences`, `reset_preferences`) in `src-tauri/src/presentation/commands.rs`

### Implementation for User Story 1

- [x] T008 [US1] Implement `ManagePreferencesUseCase` in `src-tauri/src/application/preferences.rs` and export in `src-tauri/src/application/mod.rs`
- [x] T009 [US1] Implement IPC commands in `src-tauri/src/presentation/commands.rs` and register in `src-tauri/src/lib.rs`
- [x] T010 [US1] Implement preferences bridge methods in `src/services/tauriBridge.ts` with tests in `src/services/tauriBridge.test.ts`

**Checkpoint**: User Story 1 is functional: preferences are read and persisted via IPC.

---

## Phase 4: User Story 2 - Personalización y Re-registro Dinámico de Atajos Globales (Priority: P1) 🎯 MVP

**Goal**: Validate and dynamically re-register capture shortcut on change without restarting the app.

**Independent Test**: Change `capture_shortcut`, verify old shortcut is unregistered and new shortcut is registered and triggers capture.

### Tests for User Story 2 ⚠️
> **NOTE: Write these tests FIRST, ensure they fail before implementation**

- [x] T011 [P] [US2] Write unit tests for dynamic shortcut re-registration and error rollback in `src-tauri/src/application/preferences.rs`

### Implementation for User Story 2

- [x] T012 [US2] Implement dynamic shortcut re-registration in `ManagePreferencesUseCase` and update `IGlobalShortcutDriver`

**Checkpoint**: User Stories 1 and 2 are functional: shortcuts re-register dynamically.

---

## Phase 5: User Stories 3 & 4 - Interfaz de Configuración (SettingsModal) y System Tray (Priority: P2/P3)

**Goal**: Provide accessible graphical settings dialog and connect tray menu "Preferencias..." option.

**Independent Test**: Open SettingsModal, edit values, click save, verify update is sent through bridge and toast confirms.

### Tests for User Stories 3 & 4 ⚠️

- [x] T013 [P] [US3] Write unit tests for `SettingsModal` in `src/components/SettingsModal.test.ts`

### Implementation for User Stories 3 & 4

- [x] T014 [US3] Implement `SettingsModal` component in `src/components/SettingsModal.ts`
- [x] T015 [US4] Connect System Tray "Preferencias..." option and settings toggle in main view

**Checkpoint**: Full end-to-end user experience with UI modal and tray integration.

---

## Phase 6: Polish & Quality Gates

**Purpose**: Documentation, quickstart validation, and quality gates

- [x] T016 [P] Update `README.md` with configuration options and local file path documentation
- [x] T017 Execute quickstart verification (`specs/006-settings-and-user-preferences/quickstart.md`)
- [x] T018 Run formatting, Clippy, and test suites (`cargo fmt -- --check && cargo clippy -- -D warnings && npm test`)
