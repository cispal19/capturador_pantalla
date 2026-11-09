# Quickstart: Testing User Preferences & Settings (Feature 006)

## 1. Automated Unit Tests

Execute the unit test suites:
```bash
# Frontend tests (SettingsModal and tauriBridge)
npm run test:frontend

# Backend tests (preferences domain, JSON store driver, update use case)
npm run test:backend

# Full test suite
npm test
```

---

## 2. Manual Verification Checklist

1. **Launch Application**:
   - Verify `~/.config/capturador-pantalla/config.json` (or OS equivalent) is created if not already present.
2. **Open Preferences**:
   - Click "Preferencias..." on the System Tray icon, or press the settings button on the UI.
   - Verify the Settings Modal opens displaying current preferences.
3. **Change Shortcut**:
   - Change `capture_shortcut` from `CommandOrControl+Shift+X` to `Alt+Shift+C`.
   - Click "Guardar".
   - Press `Alt+Shift+C` -> Verify interactive capture triggers immediately without restarting the app.
4. **Change Storage Directory & Format**:
   - Select `jpeg` as default format.
   - Capture a region and press `Cmd+S` -> Verify file is saved with `.jpeg` extension.
5. **Reset to Defaults**:
   - Click "Restablecer valores por defecto" -> Verify defaults are restored and persisted.
