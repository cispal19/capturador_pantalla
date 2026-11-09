# Quickstart: Output Clipboard & Local Storage Validation

**Feature**: `005-output-clipboard-and-local-storage`  
**Date**: 2026-11-09

---

## 1. Automated Test Execution (TDD Gates)

### Backend Domain, Encoding, and Clipboard Tests
```bash
npm run test:backend
# Verifies FileNameGenerator, ArboardClipboardDriver, FileSystemStorageDriver, and use cases
```

### Frontend Bridge and Output Tests
```bash
npm run test:frontend
# Verifies copyCaptureToClipboard, saveCaptureToFile, and key triggers (Enter, Cmd+S)
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

1. Launch application: `npm run tauri dev`.
2. Press `Command/Ctrl + Shift + X` to trigger capture.
3. Select an area on screen.
4. (Optional) Draw an arrow or blur a word.
5. **Test Clipboard Copy**:
   - Press `Enter` (or click "Copiar" in toolbar).
   - Overlay immediately closes.
   - Open Slack, Teams, or an image editor and press `Ctrl+V` / `Cmd+V`.
   - The annotated/censored screenshot is pasted cleanly!
6. **Test Local Save**:
   - Trigger capture again, select area.
   - Press `Ctrl+S` / `Cmd+S` (or click "Guardar").
   - File is saved in pictures/downloads directory as `Captura_YYYY-MM-DD_HH-mm-ss.png`.