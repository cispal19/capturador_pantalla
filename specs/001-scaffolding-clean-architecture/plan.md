# Implementation Plan: Project Scaffolding & Clean Architecture Foundation

**Branch**: `001-scaffolding-clean-architecture` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-scaffolding-clean-architecture/spec.md`

## Summary

This feature establishes the production-grade foundational scaffolding for the Screen Capture desktop application. It bootstraps a cross-platform desktop framework using Tauri 2.0 (Rust backend + TypeScript/Vite frontend) structured strictly under Clean Architecture. The domain core (`BoundingBox`, `DisplayCoordinates`, `RawDisplayBuffer`, `CaptureSessionState`) is isolated with zero external framework dependencies, fully verifiable in headless test environments. Automated unit test harnesses (`cargo test`, `vitest`), code quality linters, and a cross-platform CI matrix in GitHub Actions enforce the project's ratified Constitution from the initial commit.

---

## Technical Context

**Language/Version**: Rust 1.80+ (Stable) & TypeScript 5.5+ (Node.js 20+ LTS)

**Primary Dependencies**:
- Backend: Tauri 2.0 (`tauri`), `serde`, `serde_json`, `thiserror`.
- Frontend: Vite, TypeScript, `@tauri-apps/api`.

**Storage**: Local user directory only (`~/.config/capturador-pantalla/` on Linux/macOS, `%APPDATA%\capturador-pantalla` on Windows). Zero database, zero cloud storage, zero network transmission.

**Testing**:
- Backend: `cargo test` for domain logic, value object invariants, and use cases with mock drivers.
- Frontend: `vitest` for TypeScript contracts and canvas coordinate calculations.

**Target Platform**:
- macOS: 12.0 (Monterey) or higher (Apple Silicon & Intel).
- Windows: Windows 10 & 11 (64-bit).
- Linux: Ubuntu 22.04 LTS / Fedora 38+ (X11 and Wayland compatible).

**Project Type**: Desktop application (background resident process with system tray integration and transparent full-screen overlay capability).

**Performance Goals**:
- Background resident memory consumption: < 40 MB RAM in idle state.
- Domain unit test execution time: < 1 second for the entire core test suite.
- Cold development startup time: < 5 seconds.

**Constraints**:
- Strict Local-First architecture: 0 network sockets opened, 0 telemetry calls.
- Permissive corporate open-source licensing: All runtime and development dependencies must comply with MIT, Apache 2.0, or BSD licenses.

**Scale/Scope**: Foundation layer providing 5 core domain entities/value objects, 4 abstraction port contracts, and CI quality gates.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Constitutional Principle | Requirement | Plan Conformance | Status |
| :--- | :--- | :--- | :--- |
| **I. TDD (Test-Driven Development)** | Tests written before production logic; Red-Green-Refactor enforced. | Test harness configured with `cargo test` and `vitest`. Domain models include unit tests verifying boundary math and normalization before feature implementation. | **PASS** |
| **II. SOLID Design Principles** | Single Responsibility, Open/Closed, Liskov, Interface Segregation, Dependency Inversion. | Clear segregation of interfaces in `ports/`; domain models have single reasons to change; concrete adapters injected via traits. | **PASS** |
| **III. Clean Architecture** | Domain core isolated from UI, frameworks, and OS APIs; dependencies point inward. | Domain layer in `src-tauri/src/domain/` has zero dependencies on Tauri or OS libraries. Tested 100% headless. | **PASS** |
| **IV. DRY, KISS & YAGNI** | No redundant representations, no overengineering, no speculative features. | Minimalist scaffolding with only required foundation types; no premature database or multi-threaded complexity. | **PASS** |
| **V. BRD & ARD Alignment** | Adherence to business rules and architectural specifications. | Conforms directly to BR-001 (Local-First), BR-002 (Permissive Licensing), TR-001, TR-008, and TR-010. | **PASS** |
| **Section 2: Security & Dependencies** | No secrets in code; permissive licenses; SCA audit gates. | `cargo-audit` and `npm audit` configured in CI; zero network calls; no hardcoded credentials. | **PASS** |
| **Section 3: Quality Standards** | Automated tests passing 100%; code formatting; linters. | GitHub Actions CI matrix executes tests, `cargo clippy`, `rustfmt`, and `eslint` on every pull request. | **PASS** |

---

## Project Structure

### Documentation (this feature)

```text
specs/001-scaffolding-clean-architecture/
├── spec.md              # Feature specification
├── plan.md              # This implementation plan
├── research.md          # Phase 0: Technology evaluations & architectural decisions
├── data-model.md        # Phase 1: Domain entities, Value Objects & invariants
├── quickstart.md        # Phase 1: Runnable verification guide
├── contracts/           # Phase 1: Interface contracts
│   ├── ports.md         # Rust driver traits (IScreenCaptureDriver, IClipboardDriver, etc.)
│   └── ipc-commands.md  # Tauri IPC command schemas
└── checklists/
    └── requirements.md  # Requirements quality validation checklist
```

### Source Code Layout

```text
capturador_pantalla/
├── .github/
│   └── workflows/
│       └── ci.yml             # Cross-platform GitHub Actions workflow
├── src-tauri/
│   ├── Cargo.toml             # Rust package definition, dependencies & dev-dependencies
│   ├── tauri.conf.json        # Tauri 2.0 configuration (security, window definitions)
│   ├── build.rs               # Tauri build script
│   └── src/
│       ├── domain/            # Layer 1: Core Domain (Pure Rust, 0 external dependencies)
│       │   ├── mod.rs
│       │   ├── coordinates.rs # DisplayCoordinates Value Object
│       │   ├── dimensions.rs  # Dimensions Value Object
│       │   ├── bounding_box.rs# BoundingBox Value Object with normalization & clamping
│       │   ├── buffer.rs      # RawDisplayBuffer Entity
│       │   └── error.rs       # DomainError definitions
│       ├── application/       # Layer 2: Use Cases
│       │   ├── mod.rs
│       │   └── session.rs     # Capture session lifecycle management
│       ├── ports/             # Layer 3: Boundary Interfaces (DIP Traits)
│       │   ├── mod.rs
│       │   ├── capture.rs     # IScreenCaptureDriver trait
│       │   ├── clipboard.rs   # IClipboardDriver trait
│       │   ├── storage.rs     # IFileSystemDriver trait
│       │   └── shortcut.rs    # IGlobalShortcutDriver trait
│       ├── infrastructure/    # Layer 4: Concrete Adapters
│       │   └── mod.rs         # Adapter factory placeholders
│       ├── presentation/      # Layer 5: Tauri Commands & Handlers
│       │   ├── mod.rs
│       │   └── commands.rs    # Typed IPC command handlers
│       ├── lib.rs             # Application initialization & Tauri builder setup
│       └── main.rs            # Desktop entrypoint
├── src/                       # Frontend Presentation Layer (TypeScript + Vite)
│   ├── components/            # UI Components
│   ├── services/              # Tauri IPC bridge client
│   ├── types/
│   │   └── domain.ts          # TypeScript interfaces mirroring Rust domain contracts
│   ├── App.css
│   ├── App.tsx (or App.ts)    # Root presentation component
│   └── main.ts                # Frontend entrypoint
├── index.html                 # HTML shell
├── package.json               # Node.js dependencies, scripts & lint commands
├── tsconfig.json              # Strict TypeScript configuration
└── vite.config.ts             # Vite build configuration optimized for Tauri
```

**Structure Decision**:
Adopted Clean Architecture with Tauri 2.0. The backend code in `src-tauri/src/` is strictly partitioned into `domain/`, `application/`, `ports/`, `infrastructure/`, and `presentation/`. The frontend in `src/` communicates exclusively through typed IPC commands defined in `contracts/ipc-commands.md`.

---

## Complexity Tracking

> *No constitutional violations detected. All gates passed with zero exceptions.*
