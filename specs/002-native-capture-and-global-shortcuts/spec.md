# Feature Specification: Native Screen Capture Engine & Global Shortcuts

**Feature Branch**: `002-native-capture-and-global-shortcuts`

**Created**: 2026-11-09

**Status**: Ready for Planning

**Input**: User description: "002-native-capture-and-global-shortcuts - Motor de captura de pantalla nativa y gestión de atajos globales conforme a FR-001, FR-002, TR-001, TR-009 y la Constitución."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Instant Capture Trigger via Global Shortcut (Priority: P1)

As a user working in any desktop application, I want to press a global keyboard combination (such as `Cmd+Shift+X` on macOS, or `Ctrl+Shift+X` / `PrintScreen` on Windows and Linux) so that the capture workflow activates instantly without having to bring the capture application to the foreground.

**Why this priority**: Core interaction paradigm; fast, ubiquitous keyboard access is the primary value proposition of the tool.

**Independent Test**: Can be tested by registering a shortcut accelerator, triggering the key combination from an external window (e.g. terminal or browser), and verifying that the capture trigger callback fires and transitions the capture session state to `Selecting`.

**Acceptance Scenarios**:

1. **Given** the application is running in the background, **When** the user presses the registered global shortcut combination, **Then** the application triggers the screen grabber and initiates the capture session in under 150 milliseconds.
2. **Given** the global shortcut is registered, **When** pressed repeatedly, **Then** the application debounces duplicate rapid triggers and maintains session integrity.

---

### User Story 2 - High-Fidelity Multi-Monitor Screen Grabber (Priority: P2)

As a user working with one or more monitors (including high-density Retina or 4K displays), I want the system to grab the full-resolution pixel buffer of the active or all displays so that the capture preserves exact sharpness, true color representation, and accurate physical pixel dimensions.

**Why this priority**: Delivers the visual substance of the capture; captures must be crisp and accurate for developers, designers, and corporate documentation.

**Independent Test**: Can be tested by invoking the screen capture driver in a multi-monitor or single-monitor environment, verifying that the returned buffer matches the display dimensions and has a valid RGBA pixel layout with zero byte corruption.

**Acceptance Scenarios**:

1. **Given** a connected display with scale factor $S$ (e.g. 2.0 on Retina), **When** a screen capture is executed, **Then** the returned `RawDisplayBuffer` contains physical dimensions equal to $\text{width} \times S$ and $\text{height} \times S$, formatted in 32-bit RGBA without downsampling.
2. **Given** multiple displays connected, **When** `capture_all_displays` is called, **Then** individual buffers are returned for each display with their corresponding monitor identifiers and coordinates.

---

### User Story 3 - System Tray Quick Actions & OS Permissions Guidance (Priority: P3)

As a user or corporate employee, I want a permanent icon in the system tray / menu bar with a contextual menu ("Tomar Captura", "Preferencias", "Salir") and clear, non-technical guidance if the operating system requires screen recording permissions.

**Why this priority**: Essential for discoverability, background management, and graceful handling of macOS Screen Recording and Wayland portal permissions.

**Independent Test**: Can be tested by verifying tray icon initialization, clicking "Tomar Captura" to trigger the capture workflow, and testing the permission check handler when permission is missing.

**Acceptance Scenarios**:

1. **Given** the application is running, **When** the user right-clicks the tray icon, **Then** a menu appears with options to take a capture, open preferences, or exit.
2. **Given** macOS or Wayland screen recording permissions are not granted, **When** capture is attempted, **Then** the system returns a descriptive `PermissionDenied` error and presents guidance to enable permissions in system settings.

---

### Edge Cases

- **Permission Denied by OS**: When screen capture permissions are revoked or not yet granted, the system MUST return a structured `CaptureDriverError::PermissionDenied` and display a helpful dialog rather than crashing.
- **Mixed DPI Multi-Monitor Setup**: When a standard (1x) monitor is placed beside a Retina (2x) monitor, the capture engine MUST maintain independent scale factors for each buffer to prevent blurry images or misaligned bounding boxes.
- **Shortcut Already Claimed by Another Application**: If the requested accelerator is reserved or claimed by the OS or another app, registration MUST return a clean error (`ShortcutDriverError::AlreadyRegistered`), keeping the application stable.
- **Display Disconnection**: If a monitor is unplugged during operation, the capture driver MUST re-enumerate active displays and fall back to the primary display without panics.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST register global system-wide hotkeys that trigger the capture workflow regardless of which application currently has window focus.
- **FR-002**: System MUST capture full-resolution raw pixel buffers from connected monitors in memory without writing temporary image files to disk.
- **FR-003**: System MUST detect and report display metadata (identifier, dimensions, scale factor, and whether it is the primary display).
- **FR-004**: System MUST maintain a resident System Tray icon with options to initiate capture, open preferences, and terminate the application.
- **FR-005**: System MUST detect when screen capture permissions are missing and guide the user to system settings without crashing.
- **FR-006**: System MUST enforce 100% Local-First data sovereignty: captured pixel buffers MUST remain strictly in volatile memory and never be transmitted over network interfaces.

---

### Key Entities

- **DisplayInfo**: Represents metadata for a physical monitor (ID, name, logical width/height, physical width/height, scale factor, position offset, primary flag).
- **RawDisplayBuffer**: Entity encapsulating raw RGBA pixel data, physical dimensions, and byte stride.
- **GlobalShortcutBinding**: Value Object representing an accelerator string (e.g. `CommandOrControl+Shift+X`) and its associated action.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Capture latency between shortcut trigger and pixel buffer availability in memory is under 150 milliseconds.
- **SC-002**: 100% color and spatial fidelity: captured buffer matches physical display pixel density with zero compression artifacts.
- **SC-003**: 0 bytes written to disk during the capture acquisition phase (in-memory buffer pipeline).
- **SC-004**: 100% of unit and adapter contract tests pass in the automated test suite.
- **SC-005**: Idle memory consumption remains under 40 MB RAM while listening for global shortcuts.

---

## Assumptions

- Operating system allows registering global keyboard hooks (standard on macOS, Windows, and X11/Wayland desktop environments).
- On macOS, the user grants "Screen Recording" permissions when prompted by the operating system.
- Modern 64-bit multi-monitor graphics drivers are present.
