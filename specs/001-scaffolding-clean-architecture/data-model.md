# Data Model & Domain Entities: Scaffolding & Core Domain

**Feature**: `001-scaffolding-clean-architecture`  
**Date**: 2026-11-09  
**Status**: Completed  

---

## 1. Domain Entities & Value Objects Overview

All domain entities reside in `src-tauri/src/domain/` and are pure Rust data structures with zero dependencies on Tauri, Webviews, or OS graphics APIs.

```
┌────────────────────────────────────────────────────────┐
│                   CaptureSessionState                  │
│       [Idle] ──► [Selecting] ──► [Completed]          │
│                      │                                 │
│                      ▼                                 │
│                 [Cancelled]                            │
└────────────────────────────────────────────────────────┘

┌────────────────────────┐      ┌────────────────────────┐
│   DisplayCoordinates   │      │       Dimensions       │
│   - x: f64             │      │   - width: f64         │
│   - y: f64             │      │   - height: f64        │
└───────────┬────────────┘      └───────────┬────────────┘
            │                               │
            └───────────────┬───────────────┘
                            ▼
               ┌────────────────────────┐
               │      BoundingBox       │
               │   - origin: Coord      │
               │   - size: Dimensions   │
               │   - scale_factor: f64  │
               └────────────┬───────────┘
                            │
                            ▼
               ┌────────────────────────┐
               │    RawDisplayBuffer    │
               │   - width: u32         │
               │   - height: u32        │
               │   - stride: usize      │
               │   - format: PixelFormat│
               │   - data: Vec<u8>      │
               └────────────────────────┘
```

---

## 2. Detailed Specifications

### 2.1 Value Object: `DisplayCoordinates`
Represents an immutable 2D point in screen coordinate space.

- **Fields**:
  - `x`: `f64` (horizontal coordinate)
  - `y`: `f64` (vertical coordinate)
- **Invariants & Validation**:
  - `x` and `y` MUST be finite numbers (no `NaN`, no `+Infinity`, no `-Infinity`).
- **Behaviors**:
  - `translate(dx: f64, dy: f64) -> DisplayCoordinates`: Returns a new translated coordinate.
  - `distance_to(other: &DisplayCoordinates) -> f64`: Computes Euclidean distance.

### 2.2 Value Object: `Dimensions`
Represents 2D spatial dimensions (width and height).

- **Fields**:
  - `width`: `f64` (horizontal extent)
  - `height`: `f64` (vertical extent)
- **Invariants & Validation**:
  - `width >= 0.0` and `height >= 0.0`.
  - Values MUST be finite.
- **Behaviors**:
  - `is_empty() -> bool`: Returns `true` if `width == 0.0 || height == 0.0`.
  - `area() -> f64`: Computes `width * height`.

### 2.3 Value Object: `BoundingBox`
Represents a canonical normalized rectangular region for screen capture.

- **Fields**:
  - `origin`: `DisplayCoordinates` (top-left point $X_0, Y_0$)
  - `size`: `Dimensions` (width and height)
  - `scale_factor`: `f64` (monitor DPI scale factor, default `1.0`, must be $> 0.0$)
- **Invariants & Validation**:
  - Canonical representation: Origin MUST always represent the minimum coordinates (top-left).
  - An arbitrary drag from $(X_1, Y_1)$ to $(X_2, Y_2)$ in any direction is normalized automatically.
- **Key Methods**:
  - `from_points(p1: DisplayCoordinates, p2: DisplayCoordinates, scale: f64) -> Result<BoundingBox, DomainError>`:
    $$X_0 = \min(p_1.x, p_2.x), \quad Y_0 = \min(p_1.y, p_2.y)$$
    $$\text{width} = |p_2.x - p_1.x|, \quad \text{height} = |p_2.y - p_1.y|$$
  - `clamp(max_width: f64, max_height: f64) -> BoundingBox`: Constrains the bounding box to lie strictly within $(0, 0, \text{max\_width}, \text{max\_height})$.
  - `to_physical_pixels() -> (u32, u32, u32, u32)`: Multiplies coordinates and dimensions by `scale_factor` rounded to integer physical pixels.

### 2.4 Entity: `RawDisplayBuffer`
Encapsulates in-memory raw pixel data extracted from screen capture drivers.

- **Fields**:
  - `width`: `u32` (physical pixel width)
  - `height`: `u32` (physical pixel height)
  - `stride`: `usize` (byte count per row)
  - `format`: `PixelFormat` (enum: `Rgba8`, `Bgra8`)
  - `data`: `Vec<u8>` (uncompressed raw pixel buffer)
- **Invariants**:
  - `data.len() == stride * height as usize`.
  - `stride >= width as usize * 4` (for 32-bit RGBA/BGRA).

### 2.5 Enum: `CaptureSessionState`
Finite state machine representing the lifecycle of a capture session.

- **States**:
  - `Idle`: Application is resident in background listening for global hotkeys.
  - `Selecting`: Overlay is active; user is dragging/adjusting the bounding box.
  - `Completed`: Region confirmed, output generated and transferred to clipboard/disk.
  - `Cancelled`: Session aborted via `Esc` key or explicit dismissal; resources freed.

---

## 3. Serialization & Contracts Mapping

Domain entities provide serialization (`serde::Serialize`, `serde::Deserialize`) for safe IPC transmission across the Tauri boundary into TypeScript representations defined in `src/types/domain.ts`.
