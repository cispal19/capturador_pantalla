// @vitest-environment happy-dom
import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { SelectionOverlay } from "./SelectionOverlay";

describe("SelectionOverlay Component", () => {
  let container: HTMLElement;
  let overlay: SelectionOverlay;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
    overlay = new SelectionOverlay();
    overlay.mount(container);
  });

  afterEach(() => {
    overlay.destroy();
    document.body.innerHTML = "";
    vi.restoreAllMocks();
  });

  it("mounts canvas element with full-screen dimensions", () => {
    const canvas = container.querySelector("canvas");
    expect(canvas).not.toBeNull();
    expect(overlay.getState().mode).toBe("idle");
    expect(overlay.getActiveBox()).toBeNull();
  });

  it("draws initial selection on mouse drag (P1)", () => {
    const canvas = container.querySelector("canvas")!;

    // MouseDown at (100, 100)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    expect(overlay.getState().mode).toBe("drawing");

    // MouseMove to (500, 400)
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 500, clientY: 400 }));
    const box = overlay.getActiveBox();
    expect(box).toEqual({ x: 100, y: 100, width: 400, height: 300 });

    // MouseUp
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 500, clientY: 400 }));
    expect(overlay.getState().mode).toBe("selected");
  });

  it("normalizes inverted drag (bottom-right to top-left) correctly (P1)", () => {
    const canvas = container.querySelector("canvas")!;

    // MouseDown at (500, 400)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 500, clientY: 400 }));
    // MouseMove to (100, 100)
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 100, clientY: 100 }));
    // MouseUp
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 100, clientY: 100 }));

    expect(overlay.getActiveBox()).toEqual({ x: 100, y: 100, width: 400, height: 300 });
  });

  it("resizes selection using SouthEast handle (P2)", () => {
    const canvas = container.querySelector("canvas")!;

    // Create selection (100, 100, 200, 200) -> SE handle is at (300, 300)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 300, clientY: 300 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 300, clientY: 300 }));

    // Click on SE handle at (300, 300)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 300, clientY: 300 }));
    expect(overlay.getState().mode).toBe("resizing");

    // Drag SE handle +50, +30
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 350, clientY: 330 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 350, clientY: 330 }));

    expect(overlay.getActiveBox()).toEqual({ x: 100, y: 100, width: 250, height: 230 });
  });

  it("moves selection by interior drag (P2)", () => {
    const canvas = container.querySelector("canvas")!;

    // Create selection (100, 100, 200, 200)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 300, clientY: 300 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 300, clientY: 300 }));

    // Click inside the box at (150, 150)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 150, clientY: 150 }));
    expect(overlay.getState().mode).toBe("moving");

    // Drag +40, +60
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 190, clientY: 210 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 190, clientY: 210 }));

    expect(overlay.getActiveBox()).toEqual({ x: 140, y: 160, width: 200, height: 200 });
  });

  it("cancels selection and triggers onCancel when Escape key is pressed (P3)", () => {
    const onCancelMock = vi.fn();
    overlay.setOnCancel(onCancelMock);

    const canvas = container.querySelector("canvas")!;
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 300, clientY: 300 }));

    // Press Escape
    window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape" }));

    expect(onCancelMock).toHaveBeenCalledOnce();
    expect(overlay.getActiveBox()).toBeNull();
    expect(overlay.getState().mode).toBe("idle");
  });
});
