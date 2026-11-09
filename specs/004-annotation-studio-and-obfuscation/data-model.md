# Data Model: Annotation Studio & Obfuscation Engine

**Feature**: `004-annotation-studio-and-obfuscation`  
**Date**: 2026-11-09  
**Status**: Formal Data Model

---

## 1. Domain Entities & Value Objects (Rust Core)

### `AnnotationTool` (Enum)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationTool {
    Select,
    Rectangle,
    Ellipse,
    Arrow,
    Line,
    Freehand,
    Text,
    Highlighter,
    Obfuscation,
}
```

### `Annotation` (Discriminated Enum)
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Annotation {
    Rectangle {
        id: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: String,
        stroke_width: f64,
    },
    Ellipse {
        id: String,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        color: String,
        stroke_width: f64,
    },
    Arrow {
        id: String,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        color: String,
        stroke_width: f64,
    },
    Line {
        id: String,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        color: String,
        stroke_width: f64,
    },
    Freehand {
        id: String,
        points: Vec<(f64, f64)>,
        color: String,
        stroke_width: f64,
    },
    Text {
        id: String,
        x: f64,
        y: f64,
        content: String,
        font_size: f64,
        color: String,
    },
    Highlighter {
        id: String,
        points: Vec<(f64, f64)>,
        color: String,
        stroke_width: f64,
    },
    Obfuscation {
        id: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        block_size: u32,
    },
}
```

### `ObfuscationKernel` (Domain Service)
```rust
pub struct ObfuscationKernel;

impl ObfuscationKernel {
    /// Applies irreversible block-averaging pixelation to an RGBA buffer slice
    pub fn apply_pixelate(
        buffer: &mut [u8],
        stride: usize,
        rect_x: usize,
        rect_y: usize,
        rect_w: usize,
        rect_h: usize,
        block_size: usize,
    );
}
```

### `UndoRedoHistory<T>` (Domain Container)
```rust
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UndoRedoHistory<T> {
    pub undo_stack: Vec<T>,
    pub redo_stack: Vec<T>,
}

impl<T: Clone> UndoRedoHistory<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, item: T);
    pub fn undo(&mut self) -> Option<T>;
    pub fn redo(&mut self) -> Option<T>;
    pub fn can_undo(&self) -> bool;
    pub fn can_redo(&self) -> bool;
    pub fn current_items(&self) -> &[T];
    pub fn clear(&mut self);
}
```

---

## 2. Frontend TypeScript Types (`src/types/annotations.ts`)

```typescript
export type ActiveTool =
  | 'select'
  | 'rectangle'
  | 'ellipse'
  | 'arrow'
  | 'line'
  | 'freehand'
  | 'text'
  | 'highlighter'
  | 'obfuscation';

export type CorporateColor =
  | '#ef4444' // Red
  | '#10b981' // Green
  | '#3b82f6' // Blue
  | '#f59e0b' // Yellow
  | '#ffffff' // White
  | '#000000'; // Black

export type StrokeThickness = 2 | 4 | 8; // Fine, Medium, Thick

export type AnnotationItem =
  | { type: 'rectangle'; id: string; x: number; y: number; width: number; height: number; color: string; stroke_width: number }
  | { type: 'ellipse'; id: string; x: number; y: number; radius_x: number; radius_y: number; color: string; stroke_width: number }
  | { type: 'arrow'; id: string; start_x: number; start_y: number; end_x: number; end_y: number; color: string; stroke_width: number }
  | { type: 'line'; id: string; start_x: number; start_y: number; end_x: number; end_y: number; color: string; stroke_width: number }
  | { type: 'freehand'; id: string; points: Array<[number, number]>; color: string; stroke_width: number }
  | { type: 'text'; id: string; x: number; y: number; content: string; font_size: number; color: string }
  | { type: 'highlighter'; id: string; points: Array<[number, number]>; color: string; stroke_width: number }
  | { type: 'obfuscation'; id: string; x: number; y: number; width: number; height: number; block_size: number };
```

---

## 3. Mathematical Invariants

| Regla | Expresión | Descripción |
| :--- | :--- | :--- |
| **INV-OBF-01** | `block_size >= 4` | El tamaño de celda de pixelado no puede ser menor a 4 píxeles para evitar reconstrucciones de alta resolución. |
| **INV-OBF-02** | $\forall p \in \text{Block}, \text{color}(p) = \text{mean}(\text{Block})$ | Todos los píxeles dentro de la celda asumen idéntico promedio RGB. |
| **INV-HIS-01** | `push(item) => redo_stack.is_empty()` | Realizar una nueva acción purga completamente la pila de rehacer. |
| **INV-HIS-02** | `undo() => undo_stack.len() - 1 && redo_stack.len() + 1` | Deshacer transfiere el elemento al tope de la pila de rehacer. |