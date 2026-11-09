# Quickstart: Testing Screen Recording Engine (Feature 007)

## 1. Automated Unit Tests

Run the full automated test suite:
```bash
# Frontend tests (RecordingToolbar, bridge methods)
npm run test:frontend

# Backend tests (RecordingRegion, recording session manager, IPC commands)
npm run test:backend

# Full test suite
npm test
```

---

## 2. Manual Verification Workflow

1. **Launch Application**:
   - `npm run tauri dev`
2. **Select Region for Recording**:
   - Open selection overlay, delimit region (e.g., 800x600).
   - In the toolbar, click the "🎥 Grabar Video" mode toggle.
3. **Record Video**:
   - Click "● Iniciar Grabación".
   - Notice the pulsing red border around the recording zone and the live timer `00:01`, `00:02`...
4. **Pause and Resume**:
   - Click "⏸️ Pausar" -> verify timer stops and border becomes amber.
   - Click "▶️ Reanudar" -> verify timer resumes and border returns to red.
5. **Stop and Verify File**:
   - Click "⏹️ Detener".
   - Verify toast: `💾 Grabación guardada en .../Grabacion_YYYY-MM-DD_HH-mm-ss.webm`.
   - Open video file with local media player (VLC, QuickTime, browser) to confirm smooth playback.
