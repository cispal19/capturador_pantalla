// @vitest-environment happy-dom
import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { AnnotationToolbar } from "./AnnotationToolbar";

describe("AnnotationToolbar Component", () => {
  let container: HTMLElement;
  let toolbar: AnnotationToolbar;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
    toolbar = new AnnotationToolbar();
    toolbar.mount(container);
  });

  afterEach(() => {
    toolbar.destroy();
    document.body.innerHTML = "";
  });

  it("renders all tool buttons, colors, and thickness options", () => {
    const rootEl = toolbar.getElement();
    expect(rootEl).not.toBeNull();

    // Check for tools
    expect(rootEl.querySelector('[data-tool="rectangle"]')).not.toBeNull();
    expect(rootEl.querySelector('[data-tool="arrow"]')).not.toBeNull();
    expect(rootEl.querySelector('[data-tool="obfuscation"]')).not.toBeNull();
    expect(rootEl.querySelector('[data-tool="highlighter"]')).not.toBeNull();

    // Check for colors
    expect(rootEl.querySelectorAll("[data-color]").length).toBe(6);

    // Check for stroke sizes
    expect(rootEl.querySelectorAll("[data-thickness]").length).toBe(3);
  });

  it("triggers onToolChange when a tool button is clicked", () => {
    const onToolMock = vi.fn();
    toolbar.setOnToolChange(onToolMock);

    const arrowBtn = toolbar.getElement().querySelector('[data-tool="arrow"]') as HTMLButtonElement;
    arrowBtn.click();

    expect(onToolMock).toHaveBeenCalledWith("arrow");
    expect(toolbar.getActiveTool()).toBe("arrow");
  });

  it("triggers onColorChange when a color swatch is clicked", () => {
    const onColorMock = vi.fn();
    toolbar.setOnColorChange(onColorMock);

    const greenBtn = toolbar
      .getElement()
      .querySelector('[data-color="#10b981"]') as HTMLButtonElement;
    greenBtn.click();

    expect(onColorMock).toHaveBeenCalledWith("#10b981");
    expect(toolbar.getActiveColor()).toBe("#10b981");
  });

  it("computes correct floating position below or above selection box", () => {
    // Selection in center of 1920x1080 screen: (500, 300, 400, 300)
    toolbar.updatePosition({ x: 500, y: 300, width: 400, height: 300 }, 1920, 1080);
    const style = toolbar.getElement().style;
    expect(parseInt(style.top, 10)).toBeGreaterThanOrEqual(608); // 300 + 300 + 8

    // Selection at bottom of screen: (500, 800, 400, 260) -> should flip above
    toolbar.updatePosition({ x: 500, y: 800, width: 400, height: 260 }, 1920, 1080);
    expect(parseInt(style.top, 10)).toBeLessThan(800);
  });

  it("renders GIF export button and triggers onGif callback on click", () => {
    const onGifMock = vi.fn();
    toolbar.setOnGif(onGifMock);

    const gifBtn = toolbar.getElement().querySelector('[data-action="gif"]') as HTMLButtonElement;
    expect(gifBtn).not.toBeNull();
    expect(gifBtn.textContent).toContain("GIF");

    gifBtn.click();
    expect(onGifMock).toHaveBeenCalledTimes(1);
  });
});
