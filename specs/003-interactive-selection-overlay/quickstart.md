# Quickstart: Interactive Selection Overlay Validation

**Feature**: `003-interactive-selection-overlay`  
**Date**: 2026-11-09

---

## 1. Prerequisites
- Rust 1.77+ installed (`source "$HOME/.cargo/env"`).
- Node.js 18+ and npm installed.

---

## 2. Automated Test Suite Execution (TDD Gates)

### Backend Domain Geometric Tests
Execute the Rust unit tests verifying handle anchor generation, hit testing, handle resizing, clamping, and translation:
```bash
npm run test:backend
# Expected: all tests pass including BoundingBox handles and translation
```

### Frontend Domain & Canvas Overlay Tests
Execute the Vitest suite verifying the selection state machine, inverted drag normalization, and 60 FPS requestAnimationFrame rendering logic:
```bash
npm run test:frontend
# Expected: all tests pass
```

### Full Quality Gate
```bash
npm test
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
```

---

## 3. Interactive Manual Verification Steps

1. Launch application in dev mode:
   ```bash
   npm run tauri dev
   ```
2. Trigger capture via hotkey (`Cmd+Shift+X` on macOS / `Ctrl+Shift+X` on Linux/Windows) or Tray Icon -> "Tomar Captura".
3. **Verify Dimming**: The screen darkens with 40% opacity.
4. **Drag Selection**:
   - Click at `(200, 200)` and drag to `(600, 500)`.
   - Verify smooth 60 FPS clear cutout with blue border and 8 white square handles.
   - Verify badge displaying `400 x 300 px`.
5. **Inverted Drag**:
   - Drag from bottom-right `(600, 500)` up to `(200, 200)`.
   - Verify the box normalizes with identical dimensions and handles.
6. **Resize via Handles**:
   - Hover over `SE` handle (cursor becomes `nwse-resize`), drag to enlarge.
   - Hover over `N` handle (cursor becomes `ns-resize`), drag to stretch upwards.
7. **Translate Box**:
   - Click inside the box (cursor becomes `move`) and drag around. The box moves without resizing.
8. **Cancel via Esc**:
   - Press `Escape`. The overlay closes immediately, returning system control.