import type { AnnotationItem } from "../types/annotations";

export type RectangleAnnotation = Extract<AnnotationItem, { type: "rectangle" }>;
export type EllipseAnnotation = Extract<AnnotationItem, { type: "ellipse" }>;
export type ArrowAnnotation = Extract<AnnotationItem, { type: "arrow" }>;
export type LineAnnotation = Extract<AnnotationItem, { type: "line" }>;
export type FreehandAnnotation = Extract<AnnotationItem, { type: "freehand" }>;
export type TextAnnotation = Extract<AnnotationItem, { type: "text" }>;
export type HighlighterAnnotation = Extract<AnnotationItem, { type: "highlighter" }>;
export type ObfuscationAnnotation = Extract<AnnotationItem, { type: "obfuscation" }>;

export class UndoRedoStack<T> {
  private undoStack: T[] = [];
  private redoStack: T[] = [];

  push(item: T): void {
    this.undoStack.push(item);
    this.redoStack = [];
  }

  undo(): T | undefined {
    const item = this.undoStack.pop();
    if (item !== undefined) {
      this.redoStack.push(item);
    }
    return item;
  }

  redo(): T | undefined {
    const item = this.redoStack.pop();
    if (item !== undefined) {
      this.undoStack.push(item);
    }
    return item;
  }

  canUndo(): boolean {
    return this.undoStack.length > 0;
  }

  canRedo(): boolean {
    return this.redoStack.length > 0;
  }

  getItems(): T[] {
    return [...this.undoStack];
  }

  clear(): void {
    this.undoStack = [];
    this.redoStack = [];
  }
}

let idCounter = 0;
function nextId(): string {
  idCounter += 1;
  return `anno-${Date.now()}-${idCounter}`;
}

export const createAnnotation = {
  rectangle(
    x: number,
    y: number,
    width: number,
    height: number,
    color: string,
    stroke_width: number
  ): RectangleAnnotation {
    return {
      type: "rectangle",
      id: nextId(),
      x,
      y,
      width,
      height,
      color,
      stroke_width,
    };
  },

  ellipse(
    x: number,
    y: number,
    radius_x: number,
    radius_y: number,
    color: string,
    stroke_width: number
  ): EllipseAnnotation {
    return {
      type: "ellipse",
      id: nextId(),
      x,
      y,
      radius_x,
      radius_y,
      color,
      stroke_width,
    };
  },

  arrow(
    start_x: number,
    start_y: number,
    end_x: number,
    end_y: number,
    color: string,
    stroke_width: number
  ): ArrowAnnotation {
    return {
      type: "arrow",
      id: nextId(),
      start_x,
      start_y,
      end_x,
      end_y,
      color,
      stroke_width,
    };
  },

  line(
    start_x: number,
    start_y: number,
    end_x: number,
    end_y: number,
    color: string,
    stroke_width: number
  ): LineAnnotation {
    return {
      type: "line",
      id: nextId(),
      start_x,
      start_y,
      end_x,
      end_y,
      color,
      stroke_width,
    };
  },

  freehand(
    points: Array<[number, number]>,
    color: string,
    stroke_width: number
  ): FreehandAnnotation {
    return {
      type: "freehand",
      id: nextId(),
      points,
      color,
      stroke_width,
    };
  },

  text(x: number, y: number, content: string, font_size: number, color: string): TextAnnotation {
    return {
      type: "text",
      id: nextId(),
      x,
      y,
      content,
      font_size,
      color,
    };
  },

  highlighter(
    points: Array<[number, number]>,
    color: string,
    stroke_width: number
  ): HighlighterAnnotation {
    return {
      type: "highlighter",
      id: nextId(),
      points,
      color,
      stroke_width,
    };
  },

  obfuscation(
    x: number,
    y: number,
    width: number,
    height: number,
    block_size = 12
  ): ObfuscationAnnotation {
    return {
      type: "obfuscation",
      id: nextId(),
      x,
      y,
      width,
      height,
      block_size: Math.max(4, block_size),
    };
  },
};
