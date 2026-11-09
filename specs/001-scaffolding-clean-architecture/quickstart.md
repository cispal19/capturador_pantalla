# Quickstart Validation Guide: Project Scaffolding & Clean Architecture

**Feature**: `001-scaffolding-clean-architecture`  
**Date**: 2026-11-09  
**Status**: Runnable Verification Guide  

This guide provides step-by-step instructions to validate the application foundation, Clean Architecture domain isolation, test harness, and quality gates.

---

## 1. Prerequisites Setup

Ensure standard toolchains are installed on the workstation:

### 1.1 Node.js
```bash
node --version # Must be v18+ (v20+ recommended)
npm --version
```

### 1.2 Rust Toolchain (Stable)
If Rust is not yet installed:
```bash
# Via official rustup installer (recommended):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Or via Homebrew (macOS):
brew install rust
```

Verify installation:
```bash
rustc --version
cargo --version
```

---

## 2. Dependency Installation

```bash
# Install frontend tooling and Tauri CLI
npm install
```

---

## 3. Running Automated Tests (TDD Verification)

### 3.1 Rust Backend & Domain Tests (Headless)
Executes all domain entity tests, value object normalization invariants, and mock port tests in under 1 second without opening any graphical windows:
```bash
npm run test:backend
# Or directly via cargo:
cargo test --manifest-path src-tauri/Cargo.toml
```

**Expected Outcome**:
```text
running 5 tests
test domain::coordinates::tests::test_coordinate_invariants ... ok
test domain::bounding_box::tests::test_bounding_box_normalization ... ok
test domain::bounding_box::tests::test_bounding_box_clamping ... ok
test domain::buffer::tests::test_buffer_dimensions ... ok
test application::session::tests::test_session_lifecycle ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

### 3.2 Frontend Unit Tests
```bash
npm run test:frontend
# Or via vitest:
npm test
```

**Expected Outcome**: 100% of unit tests pass with zero failures.

---

## 4. Code Quality & Security Audits

Execute static analysis and dependency vulnerability scans:

```bash
# Code formatting check
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
npm run format:check

# Static analysis (Clippy & ESLint)
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
npm run lint

# Security vulnerability audit
cargo audit --file src-tauri/Cargo.lock
npm audit
```

**Expected Outcome**: Zero formatting deviations, zero clippy warnings, zero critical vulnerabilities.

---

## 5. Development Mode Launch

To verify the application launches and resides in the background ready to accept commands:

```bash
npm run tauri dev
```

**Expected Outcome**:
1. Vite dev server boots on localhost.
2. Rust backend compiles with zero warnings.
3. Desktop background application initializes with tray icon / headless runner.
4. Quitting via `Ctrl+C` or tray menu terminates with exit code 0.
