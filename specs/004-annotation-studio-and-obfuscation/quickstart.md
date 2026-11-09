# Quickstart: Annotation Studio & Obfuscation Validation

**Feature**: `004-annotation-studio-and-obfuscation`  
**Date**: 2026-11-09

---

## 1. Automated Test Execution (TDD Gates)

### Backend Domain & Obfuscation Kernel Tests
```bash
npm run test:backend
# Verifies ObfuscationKernel mathematical destruction and UndoRedoHistory
```

### Frontend Annotation Studio & Toolbar Tests
```bash
npm run test:frontend
# Verifies vector drawing, floating toolbar positioning, and Undo/Redo stack
```

### Full Quality Gate
```bash
npm test
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
npm run format:check
```

---

## 2. Interactive Manual Verification

1. Run application: `npm run tauri dev`.
2. Start capture with `Command/Ctrl + Shift + X`.
3. Select an area on the screen.
4. **Verify Floating Toolbar**: The toolbar appears adjacent to the selected rectangle.
5. **Test Vector Shapes**:
   - Select "Rectángulo", draw on canvas (red border).
   - Select "Flecha", draw an arrow pointing to an icon.
   - Select "Lápiz", draw freehand.
   - Select "Resaltador", highlight text (semi-transparent yellow).
6. **Test Obfuscation**:
   - Select "Ofuscación" / "Blur" icon.
   - Drag a rectangle over text.
   - The text is immediately pixelated into flat average color blocks.
7. **Test Undo / Redo**:
   - Press `Cmd+Z` (or click Deshacer). The pixelation disappears.
   - Press `Cmd+Shift+Z` (or click Rehacer). The pixelation reappears.