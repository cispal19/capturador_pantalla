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

  it("draws vector rectangle annotation inside active selection box (Feature 004)", () => {
    const canvas = container.querySelector("canvas")!;

    // Create selection (100, 100, 400, 300)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 500, clientY: 400 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 500, clientY: 400 }));

    // Switch to rectangle tool
    const toolbar = overlay.getToolbar();
    const rectBtn = toolbar
      .getElement()
      .querySelector('[data-tool="rectangle"]') as HTMLButtonElement;
    rectBtn.click();
    expect(toolbar.getActiveTool()).toBe("rectangle");

    // Draw rectangle annotation from (150, 150) to (250, 250)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 150, clientY: 150 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 250, clientY: 250 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 250, clientY: 250 }));

    const annos = overlay.getAnnotations();
    expect(annos).toHaveLength(1);
    expect(annos[0].type).toBe("rectangle");
  });

  it("applies obfuscation annotation and handles Undo/Redo shortcuts (Feature 004)", () => {
    const canvas = container.querySelector("canvas")!;

    // Create selection (100, 100, 400, 300)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 500, clientY: 400 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 500, clientY: 400 }));

    // Switch to obfuscation tool
    const toolbar = overlay.getToolbar();
    const obfBtn = toolbar
      .getElement()
      .querySelector('[data-tool="obfuscation"]') as HTMLButtonElement;
    obfBtn.click();

    // Draw obfuscation box from (200, 200) to (300, 250)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 200, clientY: 200 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 300, clientY: 250 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 300, clientY: 250 }));

    expect(overlay.getAnnotations()).toHaveLength(1);
    expect(overlay.getAnnotations()[0].type).toBe("obfuscation");

    // Undo via Cmd+Z
    window.dispatchEvent(new KeyboardEvent("keydown", { key: "z", metaKey: true }));
    expect(overlay.getAnnotations()).toHaveLength(0);

    // Redo via Cmd+Shift+Z
    window.dispatchEvent(new KeyboardEvent("keydown", { key: "z", metaKey: true, shiftKey: true }));
    expect(overlay.getAnnotations()).toHaveLength(1);
  });

  it("exports to clipboard on Enter key press when selection is active (Feature 005)", async () => {
    const canvas = container.querySelector("canvas")!;
    const ctx = canvas.getContext("2d");
    if (ctx) {
      ctx.getImageData = vi.fn().mockReturnValue({
        width: 100,
        height: 100,
        data: new Uint8ClampedArray(100 * 100 * 4),
      });
    }

    const exportSpy = vi.spyOn(overlay, "exportToClipboard");

    // Create selection (100, 100, 200, 200)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 300, clientY: 300 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 300, clientY: 300 }));

    expect(overlay.getActiveBox()).not.toBeNull();

    // Press Enter
    window.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter" }));

    expect(exportSpy).toHaveBeenCalledOnce();
  });

  it("exports to file on Cmd+S / Ctrl+S key press when selection is active (Feature 005)", async () => {
    const canvas = container.querySelector("canvas")!;
    const ctx = canvas.getContext("2d");
    if (ctx) {
      ctx.getImageData = vi.fn().mockReturnValue({
        width: 100,
        height: 100,
        data: new Uint8ClampedArray(100 * 100 * 4),
      });
    }

    const exportFileSpy = vi.spyOn(overlay, "exportToFile");

    // Create selection (100, 100, 200, 200)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 300, clientY: 300 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 300, clientY: 300 }));

    expect(overlay.getActiveBox()).not.toBeNull();

    // Press Cmd+S
    window.dispatchEvent(new KeyboardEvent("keydown", { key: "s", metaKey: true }));

    expect(exportFileSpy).toHaveBeenCalledOnce();
  });

  it("triggers export from toolbar buttons (Feature 005)", async () => {
    const canvas = container.querySelector("canvas")!;
    const ctx = canvas.getContext("2d");
    if (ctx) {
      ctx.getImageData = vi.fn().mockReturnValue({
        width: 100,
        height: 100,
        data: new Uint8ClampedArray(100 * 100 * 4),
      });
    }

    const exportClipSpy = vi.spyOn(overlay, "exportToClipboard");
    const exportFileSpy = vi.spyOn(overlay, "exportToFile");

    // Create selection (100, 100, 200, 200)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 300, clientY: 300 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 300, clientY: 300 }));

    const toolbarEl = overlay.getToolbar().getElement();
    const copyBtn = toolbarEl.querySelector('[data-action="copy"]') as HTMLButtonElement;
    const saveBtn = toolbarEl.querySelector('[data-action="save"]') as HTMLButtonElement;

    expect(copyBtn).not.toBeNull();
    expect(saveBtn).not.toBeNull();

    copyBtn.click();
    expect(exportClipSpy).toHaveBeenCalledOnce();

    saveBtn.click();
    expect(exportFileSpy).toHaveBeenCalledOnce();
  });

  it("initiates and controls screen recording from toolbar (Feature 007)", async () => {
    const canvas = container.querySelector("canvas")!;

    // Create selection (100, 100, 300, 200)
    canvas.dispatchEvent(new MouseEvent("mousedown", { clientX: 100, clientY: 100 }));
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: 400, clientY: 300 }));
    canvas.dispatchEvent(new MouseEvent("mouseup", { clientX: 400, clientY: 300 }));

    expect(overlay.getActiveBox()).not.toBeNull();

    const recordSpy = vi.spyOn(overlay, "startRecording");
    const toolbarEl = overlay.getToolbar().getElement();
    const recordBtn = toolbarEl.querySelector('[data-action="record"]') as HTMLButtonElement;

    expect(recordBtn).not.toBeNull();
    recordBtn.click();
    await new Promise((r) => setTimeout(r, 10));

    expect(recordSpy).toHaveBeenCalledOnce();
    expect(overlay.isRecordingActive()).toBe(true);

    const recordingToolbar = overlay.getRecordingToolbar();
    expect(recordingToolbar.getElement().style.display).not.toBe("none");

    // Stop recording
    const stopSpy = vi.spyOn(overlay, "stopRecording");
    const stopBtn = recordingToolbar
      .getElement()
      .querySelector('[data-action="stop"]') as HTMLButtonElement;
    stopBtn.click();
    await new Promise((r) => setTimeout(r, 10));

    expect(stopSpy).toHaveBeenCalledOnce();
    expect(overlay.isRecordingActive()).toBe(false);
  });
});
