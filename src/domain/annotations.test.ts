import { describe, it, expect } from "vitest";
import { UndoRedoStack, createAnnotation } from "./annotations";

describe("UndoRedoStack (Frontend)", () => {
  it("initializes empty and correctly manages push, undo and redo", () => {
    const stack = new UndoRedoStack<string>();
    expect(stack.canUndo()).toBe(false);
    expect(stack.canRedo()).toBe(false);

    stack.push("item1");
    stack.push("item2");
    expect(stack.canUndo()).toBe(true);
    expect(stack.canRedo()).toBe(false);
    expect(stack.getItems()).toEqual(["item1", "item2"]);

    const undone = stack.undo();
    expect(undone).toBe("item2");
    expect(stack.getItems()).toEqual(["item1"]);
    expect(stack.canRedo()).toBe(true);

    const redone = stack.redo();
    expect(redone).toBe("item2");
    expect(stack.getItems()).toEqual(["item1", "item2"]);
    expect(stack.canRedo()).toBe(false);
  });

  it("purges redo stack on new push", () => {
    const stack = new UndoRedoStack<number>();
    stack.push(1);
    stack.push(2);
    stack.undo();
    expect(stack.canRedo()).toBe(true);

    stack.push(3);
    expect(stack.canRedo()).toBe(false);
    expect(stack.getItems()).toEqual([1, 3]);
  });
});

describe("createAnnotation helper", () => {
  it("creates rectangle annotation with id", () => {
    const rect = createAnnotation.rectangle(10, 20, 100, 50, "#ef4444", 4);
    expect(rect.type).toBe("rectangle");
    expect(rect.x).toBe(10);
    expect(rect.width).toBe(100);
    expect(rect.color).toBe("#ef4444");
    expect(rect.stroke_width).toBe(4);
    expect(rect.id).toBeDefined();
  });

  it("creates arrow annotation with normalized endpoints", () => {
    const arrow = createAnnotation.arrow(10, 10, 100, 100, "#3b82f6", 2);
    expect(arrow.type).toBe("arrow");
    expect(arrow.start_x).toBe(10);
    expect(arrow.end_x).toBe(100);
  });

  it("creates obfuscation annotation with minimum block size 4", () => {
    const obf = createAnnotation.obfuscation(50, 50, 200, 80, 12);
    expect(obf.type).toBe("obfuscation");
    expect(obf.block_size).toBe(12);
  });
});
