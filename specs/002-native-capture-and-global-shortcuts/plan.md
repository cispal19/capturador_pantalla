# Implementation Plan: Native Screen Capture Engine & Global Shortcuts

**Branch**: `002-native-capture-and-global-shortcuts` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/002-native-capture-and-global-shortcuts/spec.md`

## Summary

This feature implements the native cross-platform screen capture engine and OS-level global keyboard shortcuts in Rust, integrating with Tauri 2.0 and complying strictly with Clean Architecture. It introduces the `DisplayInfo` domain model, wraps the native `xcap` crate behind the `IScreenCaptureDriver` port trait (with a headless `MockScreenCaptureDriver` for automated TDD testing), integrates `tauri-plugin-global-shortcut` for universal hotkey activation, and configures a persistent System Tray icon.

---

## Technical Context

**Language/Version**: Rust 1.80+ (Stable) & TypeScript 5.5+ (Node.js 20+ LTS)

**Primary Dependencies**:
- Backend: `xcap` (v0.0.14+), `tauri-plugin-global-shortcut` (v2.0), `tauri` (with `tray-icon` feature), `image` (v0.25).
- Frontend: `@tauri-apps/api`.

**Storage**: In-memory raw pixel buffers (`Vec<u8>`). Zero disk writes during screen acquisition.

**Testing**:
- Headless mock tests in Rust (`cargo test`) verifying display enumeration, pixel buffer stride calculations, and shortcut callback dispatch.
- Vitest tests in TypeScript verifying IPC command invocations.

**Target Platform**: macOS (CoreGraphics/ScreenCaptureKit), Windows (DXGI/GDI), Linux (X11/Wayland Desktop Portal).

**Performance Goals**:
- Latency between shortcut press and in-memory frame buffer availability: < 150 ms.
- Idle memory footprint: < 40 MB RAM.
- Display enumeration time: < 50 ms.

**Constraints**:
- 100% Local-First: Zero network transmission of screen buffers.
- Permissive licenses only: `xcap` (MIT), `tauri-plugin-global-shortcut` (MIT/Apache 2.0).

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Constitutional Principle | Requirement | Plan Conformance | Status |
| :--- | :--- | :--- | :--- |
| **I. TDD (Test-Driven Development)** | Tests written first; Red-Green-Refactor cycle. | `MockScreenCaptureDriver` provides deterministic test fixtures for unit tests in headless environments before concrete OS integration. | **PASS** |
| **II. SOLID Design Principles** | Dependency Inversion & Single Responsibility. | `IScreenCaptureDriver` and `IGlobalShortcutDriver` abstract all OS-level APIs; business logic does not depend on `xcap` or Tauri plugins directly. | **PASS** |
| **III. Clean Architecture** | Domain isolation from OS graphics and windowing. | `DisplayInfo` and `RawDisplayBuffer` reside in pure `domain/`; adapters reside in `infrastructure/`. | **PASS** |
| **IV. DRY, KISS & YAGNI** | No premature complexity or speculative abstractions. | Leverages battle-tested `xcap` crate instead of custom C/Win32/Wayland bindings. | **PASS** |
| **V. BRD & ARD Alignment** | Local-First, fast hotkey triggering, enterprise safety. | Conforms to rules BR-001, BR-004, BR-006, TR-001, TR-009. | **PASS** |
| **Section 2: Security & Dependencies** | Permissive licensing, no hardcoded secrets, safe local memory. | All dependencies vetted (MIT); screen buffers stored in volatile memory only. | **PASS** |
| **Section 3: Quality Standards** | Automated tests passing 100%; CI matrix. | Tested across Ubuntu, macOS, and Windows via GitHub Actions. | **PASS** |

---

## Project Structure

### Documentation (this feature)

```text
specs/002-native-capture-and-global-shortcuts/
├── spec.md              # Feature specification
├── plan.md              # This implementation plan
├── research.md          # Phase 0: xcap & global shortcut evaluation
├── data-model.md        # Phase 1: DisplayInfo and GlobalShortcutBinding
├── quickstart.md        # Phase 1: Runnable verification guide
├── contracts/           # Phase 1: Interface contracts
│   ├── capture-service.md # IScreenCaptureDriver & IGlobalShortcutDriver
│   └── ipc-commands.md  # Tauri IPC commands
└── checklists/
    └── requirements.md  # Quality validation checklist
```

### Source Code Changes

```text
capturador_pantalla/
├── src-tauri/
│   ├── Cargo.toml             # Add xcap, tauri-plugin-global-shortcut, image dependencies
│   ├── tauri.conf.json        # Configure tray icon & shortcut capabilities
│   └── src/
│       ├── domain/
│       │   ├── display.rs     # New: DisplayInfo Value Object
│       │   └── mod.rs         # Export display module
│       ├── application/
│       │   ├── capture.rs     # New: TriggerCaptureSessionUseCase
│       │   └── mod.rs
│       ├── ports/
│       │   └── mod.rs         # Update IScreenCaptureDriver with get_displays()
│       ├── infrastructure/
│       │   ├── mock_capture.rs# New: MockScreenCaptureDriver for headless tests
│       │   ├── xcap_capture.rs# New: Concrete XcapScreenCaptureDriver adapter
│       │   ├── tauri_shortcut.rs # New: Concrete TauriGlobalShortcutDriver
│       │   └── mod.rs
│       ├── presentation/
│       │   ├── commands.rs    # Add get_connected_monitors, trigger_screen_capture
│       │   └── tray.rs        # New: System Tray initialization & menu handlers
│       └── lib.rs             # Register global shortcut plugin & tray builder
├── src/
│   ├── services/
│   │   ├── tauriBridge.ts     # Add getConnectedMonitors & triggerCapture methods
│   │   └── tauriBridge.test.ts# Tests for new bridge methods
│   └── types/
│       └── domain.ts          # Add DisplayInfo interface
```

---

## Complexity Tracking

> *No constitutional violations detected. All gates passed with zero exceptions.*
