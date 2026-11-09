# Phase 0 Research: Project Scaffolding & Clean Architecture Foundation

**Feature**: `001-scaffolding-clean-architecture`  
**Date**: 2026-11-09  
**Status**: Completed / All Decisions Resolved  

---

## 1. Decision: Desktop Application Framework

### Decision
Adopt **Tauri 2.0** with a Rust backend and a lightweight TypeScript/Vite frontend.

### Rationale
- **Resource Footprint**: Background idle memory consumption is ~25-35 MB RAM, compared to >180-250 MB for Electron. This is critical for corporate developer machines running IDEs, containers, and browser instances.
- **Binary Footprint**: Compressed distributable binaries are ~10-15 MB versus >120-180 MB for Electron.
- **Memory Safety & Performance**: Rust eliminates entire classes of buffer overflow vulnerabilities when processing raw screen pixel buffers at high resolution (HiDPI/Retina 4K/5K displays).
- **Security Boundary**: Tauri 2.0 provides isolated IPC permissions, strict capability sets, and avoids bundling a Node.js runtime in production distributions.

### Alternatives Considered
- **Electron (Node.js + Chromium)**: Rejected due to prohibitive memory consumption in corporate fleets and security concerns regarding arbitrary Node.js execution in production bundles.
- **Wails (Go + Webview)**: Considered viable, but Rust provides superior native crate support for multi-monitor screen capture APIs (`xcap`, `ScreenCaptureKit`) and zero-cost abstraction for pixel manipulation.
- **Qt / C++**: Rejected due to restrictive LGPL/commercial licensing friction in corporate environments and high build complexity across platforms.

---

## 2. Decision: Clean Architecture Layer Organization in Rust & TypeScript

### Decision
Structure the codebase into strictly separated layers respecting the Dependency Inversion Principle (DIP):
```text
src-tauri/src/
├── domain/            # Pure entities, Value Objects, Domain logic (0 external dependencies)
├── application/       # Use Cases orchestrating domain logic via Ports
├── ports/             # Abstract traits/interfaces for external drivers and IO
├── infrastructure/    # Concrete adapters (Tauri plugins, arboard, xcap, filesystem)
└── presentation/      # Tauri commands, state management, IPC handlers

src/ (Frontend UI)
├── components/        # Minimalist presentation components (Canvas 2D, SVG)
├── services/          # Typed IPC bridge communicating with Tauri commands
└── types/             # Shared TypeScript interfaces mirroring domain contracts
```

### Rationale
- Complies with Principle III of the Project Constitution (Clean Architecture and Decoupling).
- The domain core can be tested in pure isolation in headless mode without initializing Webviews or OS display servers.
- Ports allow mocking OS screen capture, clipboard, and file system drivers during automated testing.

### Alternatives Considered
- **Monolithic Tauri MVC**: Placing all logic inside Tauri `#[tauri::command]` functions. Rejected because it tightly couples business rules to Tauri IPC primitives, making unit testing without the Tauri runtime impossible.

---

## 3. Decision: Testing Harness and TDD Strategy

### Decision
- **Rust Backend**: Native `cargo test` for domain unit tests, use-case tests with test doubles (mocks), and adapter integration tests.
- **TypeScript Frontend**: `vitest` with `@testing-library` for canvas math and UI state verification.
- **Execution Target**: Domain test suite MUST execute in < 1 second in headless environments.

### Rationale
- Complies with Principle I of the Project Constitution (TDD Mandatory).
- Native tooling provides rapid feedback cycles in local development and zero-overhead execution in continuous integration.

### Alternatives Considered
- **End-to-End only testing (WebDriver/Selenium)**: Rejected as the primary testing strategy due to slowness and flakiness across multi-monitor setups; E2E will only be used for final smoke verification.

---

## 4. Decision: Code Quality, Formatting & Security Audits

### Decision
Integrate automated quality gates that execute locally and in CI:
- **Rust**:
  - `rustfmt` for uniform formatting.
  - `cargo clippy -- -D warnings` for strict static analysis.
  - `cargo audit` for known CVE vulnerability scanning in dependencies.
- **TypeScript**:
  - `prettier` for consistent formatting.
  - `eslint` with strict TypeScript rules.
  - `npm audit` for frontend dependency vulnerability checks.

### Rationale
- Enforces Section II and Section III of the Constitution (mandatory security and code quality gates).
- Prevents unvetted or vulnerable packages from being committed.

---

## 5. Decision: Cross-Platform Continuous Integration (CI)

### Decision
Configure GitHub Actions with a matrix build covering:
- `macos-latest` (Apple Silicon & Intel via universal/x86_64 target)
- `windows-latest` (x86_64 MSVC)
- `ubuntu-22.04` (X11 and Wayland library prerequisites)

### Rationale
- Ensures early detection of platform-specific compilation or test regressions.
- Guarantees parity across all three supported corporate operating systems.
