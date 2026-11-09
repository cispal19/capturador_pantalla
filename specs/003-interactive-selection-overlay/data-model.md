# Data Model: Interactive Selection Overlay & Geometric Primitives

**Feature**: `003-interactive-selection-overlay`  
**Date**: 2026-11-09  
**Status**: Completed

---

## 1. Domain Entities & Value Objects (Rust Backend)

### `ResizeHandle` (Enum)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResizeHandle {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl ResizeHandle {
    pub fn cursor_style(&self) -> &'static str {
        match self {
            ResizeHandle::NorthWest | ResizeHandle::SouthEast => "nwse-resize",
            ResizeHandle::NorthEast | ResizeHandle::SouthWest => "nesw-resize",
            ResizeHandle::North | ResizeHandle::South => "ns-resize",
            ResizeHandle::East | ResizeHandle::West => "ew-resize",
        }
    }
}
```

### `HandleAnchor` (Struct)
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandleAnchor {
    pub handle: ResizeHandle,
    pub x: f64,
    pub y: f64,
    pub cursor: String,
}
```

### `BoundingBox` Extensions (Value Object)
```rust
impl BoundingBox {
    /// Generates the 8 standard handle anchors for this bounding box
    pub fn handle_anchors(&self) -> Vec<HandleAnchor>;

    /// Tests whether a logical coordinate falls within a handle's hit tolerance
    pub fn hit_test_handle(&self, px: f64, py: f64, radius: f64) -> Option<ResizeHandle>;

    /// Tests whether a logical coordinate falls inside the selection body
    pub fn contains_point(&self, px: f64, py: f64) -> bool;

    /// Resizes the bounding box from a specific handle anchor by delta (dx, dy)
    pub fn resize_by_handle(&self, handle: ResizeHandle, dx: f64, dy: f64, min_size: f64) -> Self;

    /// Translates the bounding box origin by (dx, dy) clamped to monitor boundaries
    pub fn translate(&self, dx: f64, dy: f64, max_width: f64, max_height: f64) -> Self;
}
```

---

## 2. Frontend Domain Types (TypeScript)

### `src/types/selection.ts`
```typescript
export type ResizeHandleType =
  | 'nw'
  | 'n'
  | 'ne'
  | 'e'
  | 'se'
  | 's'
  | 'sw'
  | 'w';

export interface HandlePosition {
  handle: ResizeHandleType;
  x: number;
  y: number;
  cursor: string;
}

export type SelectionInteractionState =
  | { mode: 'idle' }
  | { mode: 'drawing'; startX: number; startY: number }
  | { mode: 'selected'; box: SelectionRect }
  | { mode: 'resizing'; handle: ResizeHandleType; initialBox: SelectionRect; startX: number; startY: number }
  | { mode: 'moving'; initialBox: SelectionRect; startX: number; startY: number };

export interface SelectionRect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface SelectionOverlayConfig {
  dimOpacity: number;        // default: 0.40
  borderColor: string;       // default: '#3b82f6' (corporate blue)
  borderWidth: number;       // default: 2
  handleSize: number;        // default: 8
  handleColor: string;       // default: '#ffffff'
  handleBorderColor: string; // default: '#1d4ed8'
  badgeBgColor: string;      // default: 'rgba(15, 23, 42, 0.85)'
  badgeTextColor: string;    // default: '#ffffff'
}
```

---

## 3. Invariants & Business Rules (TDD Verifiable)

| Rule ID | Invariant | Description |
| :--- | :--- | :--- |
| **INV-001** | `width >= 10 && height >= 10` | The selection cannot be resized or clamped below 10x10 logical pixels. |
| **INV-002** | `x >= 0 && y >= 0` | The origin of the box must always remain within positive display coordinates. |
| **INV-003** | `x + width <= display_w` | The right edge cannot protrude past the display width. |
| **INV-004** | `y + height <= display_h` | The bottom edge cannot protrude past the display height. |
| **INV-005** | Normalization idempotency | Calling `normalize()` on an already canonical box produces an identical box. |
| **INV-006** | Handle invariant | All 8 handle coordinates strictly match the mathematical bounds of the rect. |