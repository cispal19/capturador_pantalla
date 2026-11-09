# Data Model: Screen Capture & Shortcut Entities

**Feature**: `002-native-capture-and-global-shortcuts`  
**Date**: 2026-11-09  
**Status**: Completed  

---

## 1. Domain Entities & Value Objects

```
┌────────────────────────────────────────────────────────┐
│                      DisplayInfo                       │
│   - id: u32                                            │
│   - name: String                                       │
│   - logical_width: u32                                 │
│   - logical_height: u32                                │
│   - physical_width: u32                                │
│   - physical_height: u32                               │
│   - scale_factor: f64                                  │
│   - is_primary: bool                                   │
└────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────┐
│                 GlobalShortcutBinding                  │
│   - accelerator: String (e.g. "CommandOrControl+Shift+X")
│   - enabled: bool                                      │
│   - action: ShortcutAction (enum)                      │
└────────────────────────────────────────────────────────┘
```

---

## 2. Detailed Specifications

### 2.1 Entity: `DisplayInfo`
Encapsulates physical and logical metadata for an individual monitor.

- **Fields**:
  - `id`: `u32` (unique display index from OS)
  - `name`: `String` (human-readable monitor label, e.g. "Color LCD", "DELL U2720Q")
  - `logical_width`: `u32` (screen width in logical points)
  - `logical_height`: `u32` (screen height in logical points)
  - `physical_width`: `u32` (screen width in native device pixels)
  - `physical_height`: `u32` (screen height in native device pixels)
  - `scale_factor`: `f64` (ratio physical / logical, e.g. 2.0 for Retina, 1.0 for standard)
  - `is_primary`: `bool` (flag indicating primary desktop monitor)
- **Validation Rules**:
  - `logical_width > 0` and `logical_height > 0`.
  - `physical_width > 0` and `physical_height > 0`.
  - `scale_factor > 0.0` and finite.

### 2.2 Value Object: `GlobalShortcutBinding`
Represents an accelerator key binding registered in the operating system.

- **Fields**:
  - `accelerator`: `String` (standardized accelerator string, e.g. `CommandOrControl+Shift+X`, `PrintScreen`)
  - `enabled`: `bool`
- **Validation Rules**:
  - Accelerator syntax must be valid (contains at least one modifier or standard functional key).

### 2.3 Integration with `RawDisplayBuffer`
When captured, each monitor produces an immutable `RawDisplayBuffer`:
- `width`: matches `DisplayInfo.physical_width`.
- `height`: matches `DisplayInfo.physical_height`.
- `stride`: `physical_width * 4`.
- `format`: `PixelFormat::Rgba8`.
- `data`: vector of `width * height * 4` bytes.
