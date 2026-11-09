# Implementation Plan: Annotation Studio & Obfuscation Engine

**Branch**: `004-annotation-studio-and-obfuscation` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/004-annotation-studio-and-obfuscation/spec.md`

---

## Summary

Implement an in-memory annotation studio and irreversible obfuscation engine. This provides a dynamic floating toolbar attached to the user's selection, offering vector shapes (rectangle, ellipse, arrow, line, freehand pencil, typography, semi-transparent highlighter), corporate color and stroke selectors, a mathematically irreversible block-averaging pixelation tool (`ObfuscationKernel`) for corporate secrets/PII, and a full Undo/Redo history stack (`UndoRedoHistory`).

---

## Technical Context

**Language/Version**: Rust 1.77+ (Backend) / TypeScript 5.3+ (Frontend)  
**Primary Dependencies**: HTML5 Canvas 2D API (`requestAnimationFrame`), Tauri 2.0, Vitest, Cargo  
**Storage**: N/A (Pure memory, Local-First)  
**Testing**: `cargo test` (Rust unit tests for ObfuscationKernel and UndoRedoHistory), `vitest` (frontend Canvas and Toolbar tests)  
**Target Platform**: macOS, Windows, Linux  
**Performance Goals**: 60 FPS vector drawing; obfuscation kernel execution < 2ms per 500x500 region  
**Constraints**: 100% Local-First; irreversible pixelation destroying high-frequency data  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Plan Compliance | Status |
| :--- | :--- | :--- | :--- |
| **Principle I: TDD** | Tests written first in Red state | Mathematical kernel, history container, vector generators, and toolbar positioning tested before wiring. | **PASS** |
| **Principle II: SOLID** | Single Responsibility & Interface Segregation | `ObfuscationKernel` handles image degradation; `UndoRedoHistory` manages state; `AnnotationToolbar` handles UI presentation. | **PASS** |
| **Principle III: Clean Architecture** | Pure domain, zero UI leaks | Anotation types and history are pure models in `domain/annotation.rs` and `src/domain/annotations.ts`. | **PASS** |
| **Principle IV: DRY & KISS** | Minimal necessary complexity | Uniform discriminated union for all annotation types; single block-averaging kernel. | **PASS** |
| **Principle V: Security & Governance** | Data Protection & PII safety | Irreversible block-averaging destroys textual features with mathematical guarantee. | **PASS** |

---

## Project Structure

### Documentation

```text
specs/004-annotation-studio-and-obfuscation/
├── spec.md              # Feature specification
├── plan.md              # Implementation plan
├── research.md          # Obfuscation algorithm research & irreversibility proof
├── data-model.md        # Annotation and UndoRedo entities
├── contracts/           # IPC sync contracts
│   └── annotation-service.md
├── quickstart.md        # Test & manual validation guide
└── tasks.md             # TDD tasks list
```

### Source Code

```text
capturador_pantalla/
├── src-tauri/src/
│   ├── domain/
│   │   ├── annotation.rs       # Added: Annotation enum, UndoRedoHistory
│   │   ├── obfuscation.rs      # Added: ObfuscationKernel (block-averaging pixelation)
│   │   └── mod.rs              # Export domain models
│   ├── presentation/
│   │   └── commands.rs         # Added: apply_obfuscation, sync_annotations
│   └── lib.rs                  # Register IPC commands
├── src/
│   ├── types/
│   │   └── annotations.ts      # AnnotationItem, ActiveTool, Color, Thickness
│   ├── domain/
│   │   ├── annotations.ts      # Pure TS UndoRedoStack and shape generators
│   │   ├── annotations.test.ts # Unit tests for history and shapes
│   │   ├── obfuscation.ts      # Client-side 60 FPS canvas pixelation helper
│   │   └── obfuscation.test.ts # Client-side pixelation tests
│   ├── components/
│   │   ├── AnnotationToolbar.ts      # Floating toolbar attached to selection
│   │   ├── AnnotationToolbar.test.ts # Toolbar positioning & tool switching tests
│   │   └── SelectionOverlay.ts       # Enhanced to render active annotations & toolbar
│   └── services/
│       ├── tauriBridge.ts      # Added: applyObfuscation, syncAnnotations
│       └── tauriBridge.test.ts # IPC bridge tests
```

---

## Architecture & Implementation Flow

### Phase 1: Pure Domain Geometry & Obfuscation Kernel (TDD)
1. Write Rust unit tests for `ObfuscationKernel` (block averaging, boundary safety, irreversibility).
2. Implement `ObfuscationKernel` in `src-tauri/src/domain/obfuscation.rs`.
3. Write Rust unit tests for `UndoRedoHistory` in `src-tauri/src/domain/annotation.rs`.
4. Implement `UndoRedoHistory` in `src-tauri/src/domain/annotation.rs`.
5. Write TypeScript unit tests for `UndoRedoStack` and obfuscation in `src/domain/annotations.test.ts`.
6. Implement `src/domain/annotations.ts` and `src/domain/obfuscation.ts`.

### Phase 2: Vector Drawing & Floating Toolbar Component (TDD)
1. Write tests for `AnnotationToolbar` (attachment positioning, color selection, tool change).
2. Implement `AnnotationToolbar` in `src/components/AnnotationToolbar.ts`.
3. Integrate toolbar and vector rendering into `SelectionOverlay.ts`.

### Phase 3: Quality Gates & Verification
1. Run `cargo test`, `vitest run`, `cargo clippy`, `cargo fmt`, `prettier`.
2. Verify all quality gates pass.