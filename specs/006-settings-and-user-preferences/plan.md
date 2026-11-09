# Implementation Plan: 006-settings-and-user-preferences

## 1. Technical Context

- **Platform Target**: Tauri 2.0 (macOS, Linux, Windows), Local-First, Zero-Root / Zero-Telemetry.
- **Language / Framework**: Rust 2021 edition (Backend) + TypeScript 5 (Frontend).
- **Core Dependencies**: `serde`, `serde_json`, `tauri-plugin-global-shortcut`.
- **Storage Path**:
  - Unix/macOS: `$HOME/.config/capturador-pantalla/config.json`.
  - Windows: `%APPDATA%\capturador-pantalla\config.json`.

---

## 2. Constitution & Quality Gates Evaluation

- **Principle I (TDD)**: Every struct, method, serialization, and IPC command will have failing unit tests written prior to implementation.
- **Principle II (SOLID & Clean Architecture)**:
  - `domain::preferences`: Pure domain model `AppPreferences` and validation logic.
  - `ports::IPreferencesStoreDriver`: Trait for loading and storing preferences.
  - `infrastructure::config_store`: Concrete adapter reading/writing JSON via `IFileSystemDriver`.
  - `application::preferences`: `ManagePreferencesUseCase` orchestrating loading, updating, and re-registering shortcuts.
  - `presentation::commands`: IPC commands `get_preferences`, `update_preferences`, `reset_preferences`.
  - `src/components/SettingsModal`: Modular frontend component with accessibility and corporate styling.
- **Principle III (Zero External Telemetry / Local-First)**: 100% of preferences remain in user-space file storage.

---

## 3. Layer Architecture & File Structure

```text
src-tauri/
├── src/
│   ├── domain/
│   │   ├── preferences.rs           # AppPreferences entity, validation, defaults
│   │   └── mod.rs                  # Export domain preferences
│   ├── ports/
│   │   └── mod.rs                  # IPreferencesStoreDriver trait (optional or use IFileSystemDriver)
│   ├── infrastructure/
│   │   ├── config_store.rs          # JsonConfigStoreDriver implementing preferences persistence
│   │   └── mod.rs
│   ├── application/
│   │   ├── preferences.rs           # ManagePreferencesUseCase (load, update, re-register shortcut)
│   │   └── mod.rs
│   ├── presentation/
│   │   ├── commands.rs              # get_preferences, update_preferences, reset_preferences IPC
│   │   ├── tray.rs                  # Connect "preferences" tray item
│   │   └── mod.rs
src/
├── components/
│   ├── SettingsModal.ts             # Interactive preferences modal
│   └── SettingsModal.test.ts        # Unit tests for settings modal
├── services/
│   ├── tauriBridge.ts               # IPC wrapper for preferences
│   └── tauriBridge.test.ts          # Bridge tests
└── types/
    └── preferences.ts               # TypeScript interfaces
```

---

## 4. TDD Execution Sequence

1. **Step 1 (Red)**: Write unit tests for `AppPreferences` (defaults, serde serialization, deserialization fallback, validation) in `src-tauri/src/domain/preferences.rs`.
2. **Step 2 (Green)**: Implement `AppPreferences` in `src-tauri/src/domain/preferences.rs`.
3. **Step 3 (Red)**: Write unit tests for `JsonConfigStoreDriver` (load, save, corrupted file handling) in `src-tauri/src/infrastructure/config_store.rs`.
4. **Step 4 (Green)**: Implement `JsonConfigStoreDriver` in `src-tauri/src/infrastructure/config_store.rs`.
5. **Step 5 (Red)**: Write unit tests for `ManagePreferencesUseCase` (update with dynamic shortcut registration) in `src-tauri/src/application/preferences.rs`.
6. **Step 6 (Green)**: Implement `ManagePreferencesUseCase` in `src-tauri/src/application/preferences.rs`.
7. **Step 7 (Red & Green)**: Add IPC commands in `src-tauri/src/presentation/commands.rs` with tests and register in `src-tauri/src/lib.rs`.
8. **Step 8 (Red & Green)**: Implement TypeScript `AppPreferences` types and bridge methods in `src/services/tauriBridge.ts`.
9. **Step 9 (Red & Green)**: Implement `SettingsModal.ts` with tests in `src/components/SettingsModal.test.ts`.
10. **Step 10**: Wire tray menu and overlay trigger for preferences.
11. **Step 11**: Full verification (`npm test`, `cargo clippy`, `cargo fmt`, `npm run format:check`).
