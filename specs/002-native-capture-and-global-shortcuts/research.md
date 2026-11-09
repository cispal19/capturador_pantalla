# Phase 0 Research: Native Screen Capture Engine & Global Shortcuts

**Feature**: `002-native-capture-and-global-shortcuts`  
**Date**: 2026-11-09  
**Status**: Completed  

---

## 1. Decision: Cross-Platform Native Screen Capture Engine

### Decision
Adopt the **`xcap`** Rust crate (version `0.0.14+`, MIT License) wrapped behind our existing `IScreenCaptureDriver` port trait.

### Rationale
- **Universal Multi-OS Support**:
  - **macOS**: Utilizes `CoreGraphics` / `ScreenCaptureKit` to grab raw frame buffers from `CGDisplayStream` / `CGWindowListCreateImage`.
  - **Windows**: Utilizes Desktop Duplication API (DXGI) with fallback to GDI for high-speed DirectX-accelerated frame grabbing.
  - **Linux**: Supports X11 (`xlib`/`xcb`) and Wayland via Desktop Portal (`org.freedesktop.portal.ScreenCast`).
- **Memory Safety & In-Memory Pipeline**: Returns `image::RgbaImage` directly in memory without writing temporary files to disk, adhering to Rule **BR-001** and Rule **BR-006**.
- **Permissive License**: MIT License complies with Section 2 of the Constitution.

### Alternatives Considered
- **Handwritten OS-specific C/Objective-C/Win32 bindings**: High maintenance overhead; reimplementing Wayland PipeWire portals and DXGI duplication from scratch violates KISS and YAGNI.
- **Web-based `getDisplayMedia` via Webview**: Rejected because it requires browser permission dialogs on every capture, does not work when window is hidden, and adds unacceptable latency (>500 ms).

---

## 2. Decision: Global Shortcut Management

### Decision
Use the official **`tauri-plugin-global-shortcut`** (Tauri 2.0 compatible, MIT License) wrapped behind the `IGlobalShortcutDriver` trait.

### Rationale
- Deep integration with Tauri 2.0 application event loop.
- Supports cross-platform accelerator strings (`CommandOrControl+Shift+X`, `PrintScreen`).
- Zero CPU usage when idle: registers OS-level hotkey hooks without active polling loops.

### Alternatives Considered
- **Low-level keyboard polling thread**: Highly inefficient, drains battery, flagged by corporate antivirus/EDR software as keylogger behavior.
- **Direct OS hooks (`rdev`)**: Less integrated with Tauri's main thread and window management.

---

## 3. Decision: System Tray & Permissions Handling

### Decision
- Use Tauri 2.0's native `tauri::tray::TrayIconBuilder` with a platform-adaptive menu:
  - "Tomar Captura" (`CommandOrControl+Shift+X`)
  - "Preferencias..."
  - Separator
  - "Salir"
- On macOS, detect missing Screen Recording permissions via `CoreGraphics` access test (`CGPreflightScreenCaptureAccess` / `CGRequestScreenCaptureAccess`), returning `CaptureDriverError::PermissionDenied` with guided UI instructions.

---

## 4. Decision: Testing Strategy for Screen Capture (TDD)

### Decision
- Create a `MockScreenCaptureDriver` in `ports/` implementing `IScreenCaptureDriver` that generates synthetic test patterns (colored gradients and test bars) with arbitrary resolutions and scale factors (e.g. 1920x1080 1x, 3840x2160 2x).
- All unit and use-case tests execute against the mock driver in headless mode in continuous integration without requiring a physical monitor or display server.
- The concrete `XcapScreenCaptureDriver` is tested via integration tests when a display server is present.
