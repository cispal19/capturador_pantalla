# Quickstart: Testing Animated GIF Recording (Feature 008)

## 1. Automated Unit Tests

Execute the automated test suites:
```bash
# Frontend tests (GIF export bridge and toolbar actions)
npm run test:frontend

# Backend tests (GIF config, frame validation, and encoding)
npm run test:backend

# Full test suite
npm test
```

---

## 2. Manual Verification Workflow

1. **Launch Dev Environment**:
   - `npm run tauri dev`
2. **Select Region**:
   - Delimit a region (e.g., 300x200).
   - In the toolbar, click "🎞️ GIF".
3. **Record Sequence**:
   - Click "● Iniciar GIF".
   - Interact with the screen for 3-5 seconds.
   - Click "⏹️ Detener".
4. **Inspect Generated File**:
   - Verify toast message with output `.gif` file path.
   - Open generated GIF in a browser, Slack or image viewer to verify looping animation.
