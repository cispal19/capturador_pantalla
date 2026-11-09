# Implementation Plan: Interactive Selection Overlay & Resizing

**Branch**: `003-interactive-selection-overlay` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/003-interactive-selection-overlay/spec.md`

---

## Summary

Implement a full-screen, hardware-accelerated interactive selection overlay using HTML5 Canvas 2D and Tauri 2.0. The overlay dims the background to 40% opacity, allows drag-to-select in any cardinal direction with instantaneous coordinate normalization, renders an active rectangular bounding box with 8 interactive resize handles, real-time dimension indicators (`W x H px`), translation by interior drag, and instant session cancellation via `Escape`.

---

## Technical Context

**Language/Version**: Rust 1.77+ (Backend) / TypeScript 5.3+ (Frontend)  
**Primary Dependencies**: Tauri 2.0, HTML5 Canvas 2D API (`requestAnimationFrame`), Vitest, Cargo  
**Storage**: N/A (Pure memory, Local-First)  
**Testing**: `cargo test` (headless Rust unit tests), `vitest` (frontend unit tests)  
**Target Platform**: macOS (Darwin aarch64/x86_64), Windows (x64), Linux (x64)  
**Project Type**: Desktop Application (Tauri 2.0 + Clean Architecture)  
**Performance Goals**: 60 FPS rendering (< 16.6ms latency per frame) on 1080p, 4K, and Retina displays  
**Constraints**: Zero network telemetry (Local-First), cancel on `Esc` in < 100ms, pure TDD  
**Scale/Scope**: Multi-monitor desktop, single full-screen overlay per active monitor  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Plan Compliance | Status |
| :--- | :--- | :--- | :--- |
| **Principle I: TDD** | Tests written first in Red state | All mathematical geometry, normalization, handle positioning, hit testing, and state transitions tested before UI wiring. | **PASS** |
| **Principle II: SOLID** | Single responsibility & interface segregation | `BoundingBox` models pure geometry; Canvas overlay handles rendering; Tauri bridge handles IPC. | **PASS** |
| **Principle III: Clean Architecture** | Pure domain, no framework leakage | Domain geometric entities in Rust and TypeScript are 100% pure functions/methods with 0 external dependencies. | **PASS** |
| **Principle IV: DRY & KISS** | Minimal necessary abstractions | 8 handles parameterized uniformly by cardinal direction; single canvas surface for dimming and cutout. | **PASS** |
| **Principle V: Dependency Management** | No unvetted third-party packages | Uses native Canvas 2D API and existing Tauri runtime plugins; zero extra npm packages. | **PASS** |
| **Principle VI: Security & Governance** | Zero data exfiltration | Local-first; no coordinates or pixel buffers leave the host machine. | **PASS** |

---

## Project Structure

### Documentation (this feature)

```text
specs/003-interactive-selection-overlay/
├── spec.md              # Feature specification
├── plan.md              # This implementation plan
├── research.md          # Canvas 2D 60fps & FSM hit-testing research
├── data-model.md        # Geometric primitives and SelectionInteractionState
├── contracts/           # IPC commands and cancellation events
│   └── selection-service.md
├── quickstart.md        # Step-by-step verification guide
└── tasks.md             # Actionable TDD tasks (/speckit-tasks output)
```

### Source Code

```text
capturador_pantalla/
├── src-tauri/src/
│   ├── domain/
│   │   ├── bounding_box.rs    # Added: handle_anchors, resize_by_handle, translate, hit_test
│   │   ├── handle.rs          # Added: ResizeHandle enum and cursor styles
│   │   └── mod.rs             # Export handle and bounding box extensions
│   ├── application/
│   │   ├── session.rs         # Session lifecycle (Idle <-> Selecting <-> Annotating)
│   │   └── selection.rs       # Added: CommitSelectionUseCase, CancelSessionUseCase
│   ├── presentation/
│   │   └── commands.rs        # Added: cancel_capture_session, commit_selection_region
│   └── lib.rs                 # Register new Tauri IPC commands
├── src/
│   ├── domain/
│   │   ├── geometry.ts        # Pure TS geometric math: normalize, clamp, handleAnchors, hitTest
│   │   └── geometry.test.ts   # Unit tests for geometric math
│   ├── components/
│   │   ├── SelectionOverlay.ts      # Canvas 2D 60 FPS selection renderer & FSM event handler
│   │   └── SelectionOverlay.test.ts # Headless DOM/Canvas unit tests
│   ├── services/
│   │   ├── tauriBridge.ts     # Added: cancelCaptureSession, commitSelectionRegion
│   │   └── tauriBridge.test.ts# Updated with new IPC mocks
│   └── main.ts                # Mounts SelectionOverlay on capture trigger
```

---

## Architecture & Implementation Flow

### Phase 1: Pure Geometric Domain (TDD)
1. Write Rust unit tests for `ResizeHandle` and `BoundingBox` extensions (`handle_anchors`, `hit_test_handle`, `resize_by_handle`, `translate`).
2. Implement `handle.rs` and update `bounding_box.rs` to pass all tests in green.
3. Write TypeScript unit tests in `src/domain/geometry.test.ts` for the frontend counterpart.
4. Implement `src/domain/geometry.ts` to pass frontend unit tests in green.

### Phase 2: Application Use Cases & IPC Commands (TDD)
1. Write Rust unit tests for `cancel_capture_session` and `commit_selection_region`.
2. Implement IPC commands in `src-tauri/src/presentation/commands.rs`.
3. Write TypeScript unit tests for `tauriBridge.ts` covering cancel and commit.
4. Implement bridge methods in `src/services/tauriBridge.ts`.

### Phase 3: Hardware-Accelerated Canvas Overlay (60 FPS)
1. Write tests for `SelectionOverlay` interaction state machine (`Drawing`, `Resizing`, `Moving`, `Escape`).
2. Implement Canvas 2D rendering loop with dimming cutout, 8 handles, and dimension badge.
3. Wire `Escape` key event listener for instantaneous session cancellation.

### Phase 4: Polish & Quality Gates
1. Run `cargo test`, `vitest run`, `cargo clippy`, `cargo fmt`.
2. Verify all quality gates pass 100%.