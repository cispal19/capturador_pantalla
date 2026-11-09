import {
  DEFAULT_OVERLAY_CONFIG,
  type ResizeHandleType,
  type SelectionInteractionState,
  type SelectionOverlayConfig,
  type SelectionRect,
} from "../types/selection";
import {
  getHandleAnchors,
  hitTestHandle,
  normalizeSelection,
  pointInRect,
  resizeByHandle,
  translateRect,
} from "../domain/geometry";
import { tauriBridge } from "../services/tauriBridge";

export class SelectionOverlay {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D | null = null;
  private config: SelectionOverlayConfig;
  private state: SelectionInteractionState = { mode: "idle" };
  private activeBox: SelectionRect | null = null;
  private container: HTMLElement | null = null;
  private animationFrameId: number | null = null;
  private onCancelCallback?: () => void;
  private onSelectionCompleteCallback?: (box: SelectionRect) => void;

  constructor(config: Partial<SelectionOverlayConfig> = {}) {
    this.config = { ...DEFAULT_OVERLAY_CONFIG, ...config };
    this.canvas = document.createElement("canvas");
    this.canvas.style.position = "fixed";
    this.canvas.style.top = "0";
    this.canvas.style.left = "0";
    this.canvas.style.width = "100vw";
    this.canvas.style.height = "100vh";
    this.canvas.style.cursor = "crosshair";
    this.canvas.style.zIndex = "999999";
    this.canvas.style.pointerEvents = "auto";

    this.handleMouseDown = this.handleMouseDown.bind(this);
    this.handleMouseMove = this.handleMouseMove.bind(this);
    this.handleMouseUp = this.handleMouseUp.bind(this);
    this.handleKeyDown = this.handleKeyDown.bind(this);
    this.handleResize = this.handleResize.bind(this);
    this.render = this.render.bind(this);
  }

  mount(container: HTMLElement = document.body) {
    this.container = container;
    this.container.appendChild(this.canvas);
    this.ctx = this.canvas.getContext("2d");

    this.updateCanvasDimensions();

    this.canvas.addEventListener("mousedown", this.handleMouseDown);
    this.canvas.addEventListener("mousemove", this.handleMouseMove);
    this.canvas.addEventListener("mouseup", this.handleMouseUp);
    window.addEventListener("mouseup", this.handleMouseUp);
    window.addEventListener("keydown", this.handleKeyDown);
    window.addEventListener("resize", this.handleResize);

    this.requestRender();
  }

  destroy() {
    if (this.animationFrameId !== null) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = null;
    }

    this.canvas.removeEventListener("mousedown", this.handleMouseDown);
    this.canvas.removeEventListener("mousemove", this.handleMouseMove);
    this.canvas.removeEventListener("mouseup", this.handleMouseUp);
    window.removeEventListener("mouseup", this.handleMouseUp);
    window.removeEventListener("keydown", this.handleKeyDown);
    window.removeEventListener("resize", this.handleResize);

    if (this.canvas.parentNode) {
      this.canvas.parentNode.removeChild(this.canvas);
    }
  }

  setOnCancel(cb: () => void) {
    this.onCancelCallback = cb;
  }

  setOnSelectionComplete(cb: (box: SelectionRect) => void) {
    this.onSelectionCompleteCallback = cb;
  }

  getState(): SelectionInteractionState {
    return this.state;
  }

  getActiveBox(): SelectionRect | null {
    return this.activeBox;
  }

  cancel() {
    this.activeBox = null;
    this.state = { mode: "idle" };
    this.canvas.style.cursor = "crosshair";
    this.requestRender();

    void tauriBridge.cancelCaptureSession();

    if (this.onCancelCallback) {
      this.onCancelCallback();
    }
  }

  private updateCanvasDimensions() {
    const dpr = typeof window !== "undefined" ? window.devicePixelRatio || 1 : 1;
    const width = typeof window !== "undefined" ? window.innerWidth || 800 : 800;
    const height = typeof window !== "undefined" ? window.innerHeight || 600 : 600;

    this.canvas.width = width * dpr;
    this.canvas.height = height * dpr;

    if (this.ctx) {
      this.ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    }
  }

  private handleResize() {
    this.updateCanvasDimensions();
    this.requestRender();
  }

  private handleMouseDown(e: MouseEvent) {
    const x = e.clientX;
    const y = e.clientY;

    if (this.activeBox) {
      // 1. Check if clicking on an interactive handle
      const hitHandle = hitTestHandle(this.activeBox, x, y, this.config.handleSize);
      if (hitHandle) {
        this.state = {
          mode: "resizing",
          handle: hitHandle,
          initialBox: { ...this.activeBox },
          startX: x,
          startY: y,
          currentX: x,
          currentY: y,
        };
        this.requestRender();
        return;
      }

      // 2. Check if clicking inside selection box (translate)
      if (pointInRect(this.activeBox, x, y)) {
        this.state = {
          mode: "moving",
          initialBox: { ...this.activeBox },
          startX: x,
          startY: y,
          currentX: x,
          currentY: y,
        };
        this.canvas.style.cursor = "move";
        this.requestRender();
        return;
      }
    }

    // 3. Otherwise start a new selection drag
    this.activeBox = null;
    this.state = {
      mode: "drawing",
      startX: x,
      startY: y,
      currentX: x,
      currentY: y,
    };
    this.canvas.style.cursor = "crosshair";
    this.requestRender();
  }

  private handleMouseMove(e: MouseEvent) {
    const x = e.clientX;
    const y = e.clientY;

    switch (this.state.mode) {
      case "drawing": {
        this.activeBox = normalizeSelection(this.state.startX, this.state.startY, x, y);
        this.requestRender();
        break;
      }
      case "resizing": {
        const dx = x - this.state.startX;
        const dy = y - this.state.startY;
        this.activeBox = resizeByHandle(this.state.initialBox, this.state.handle, dx, dy);
        this.requestRender();
        break;
      }
      case "moving": {
        const dx = x - this.state.startX;
        const dy = y - this.state.startY;
        const maxW = typeof window !== "undefined" ? window.innerWidth : 1920;
        const maxH = typeof window !== "undefined" ? window.innerHeight : 1080;
        this.activeBox = translateRect(this.state.initialBox, dx, dy, maxW, maxH);
        this.requestRender();
        break;
      }
      case "selected": {
        if (this.activeBox) {
          const hit = hitTestHandle(this.activeBox, x, y, this.config.handleSize);
          if (hit) {
            this.canvas.style.cursor = this.getCursorForHandle(hit);
          } else if (pointInRect(this.activeBox, x, y)) {
            this.canvas.style.cursor = "move";
          } else {
            this.canvas.style.cursor = "crosshair";
          }
        }
        break;
      }
    }
  }

  private handleMouseUp(e: MouseEvent) {
    if (
      this.state.mode === "drawing" ||
      this.state.mode === "resizing" ||
      this.state.mode === "moving"
    ) {
      if (this.activeBox && this.activeBox.width >= 10 && this.activeBox.height >= 10) {
        this.state = { mode: "selected", box: { ...this.activeBox } };
        void tauriBridge.commitSelectionRegion(
          this.activeBox.x,
          this.activeBox.y,
          this.activeBox.width,
          this.activeBox.height
        );
        if (this.onSelectionCompleteCallback) {
          this.onSelectionCompleteCallback(this.activeBox);
        }
      } else {
        // Area was too small, cancel / reset selection
        this.activeBox = null;
        this.state = { mode: "idle" };
        this.canvas.style.cursor = "crosshair";
      }
      this.requestRender();
    }
  }

  private handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      this.cancel();
    }
  }

  private getCursorForHandle(handle: ResizeHandleType): string {
    switch (handle) {
      case "nw":
      case "se":
        return "nwse-resize";
      case "ne":
      case "sw":
        return "nesw-resize";
      case "n":
      case "s":
        return "ns-resize";
      case "e":
      case "w":
        return "ew-resize";
    }
  }

  private requestRender() {
    if (this.animationFrameId === null) {
      this.animationFrameId = requestAnimationFrame(this.render);
    }
  }

  private render() {
    this.animationFrameId = null;
    if (!this.ctx) return;

    const width = typeof window !== "undefined" ? window.innerWidth || 800 : 800;
    const height = typeof window !== "undefined" ? window.innerHeight || 600 : 600;

    // Clear whole canvas
    this.ctx.clearRect(0, 0, width, height);

    // 1. Draw 40% dark dimming overlay across the entire window
    this.ctx.fillStyle = `rgba(0, 0, 0, ${this.config.dimOpacity})`;
    this.ctx.fillRect(0, 0, width, height);

    if (this.activeBox && this.activeBox.width > 0 && this.activeBox.height > 0) {
      const { x, y, width: bw, height: bh } = this.activeBox;

      // 2. Clear out selection rectangle (clear cut-out)
      this.ctx.clearRect(x, y, bw, bh);

      // 3. Draw bounding box border
      this.ctx.strokeStyle = this.config.borderColor;
      this.ctx.lineWidth = this.config.borderWidth;
      this.ctx.strokeRect(x, y, bw, bh);

      // 4. Draw 8 resize handles
      const anchors = getHandleAnchors(this.activeBox);
      const halfSize = this.config.handleSize / 2;

      for (const anchor of anchors) {
        this.ctx.fillStyle = this.config.handleColor;
        this.ctx.fillRect(
          anchor.x - halfSize,
          anchor.y - halfSize,
          this.config.handleSize,
          this.config.handleSize
        );
        this.ctx.strokeStyle = this.config.handleBorderColor;
        this.ctx.lineWidth = 1;
        this.ctx.strokeRect(
          anchor.x - halfSize,
          anchor.y - halfSize,
          this.config.handleSize,
          this.config.handleSize
        );
      }

      // 5. Draw dimensions badge
      const badgeText = `${Math.round(bw)} × ${Math.round(bh)} px`;
      this.ctx.font = "12px system-ui, -apple-system, sans-serif";
      const textMetrics = this.ctx.measureText(badgeText);
      const badgeW = textMetrics.width + 16;
      const badgeH = 22;

      let badgeX = x;
      let badgeY = y - badgeH - 6;
      if (badgeY < 4) {
        badgeY = y + 6;
      }
      if (badgeX + badgeW > width) {
        badgeX = width - badgeW - 6;
      }

      this.ctx.fillStyle = this.config.badgeBgColor;
      this.ctx.fillRect(badgeX, badgeY, badgeW, badgeH);

      this.ctx.fillStyle = this.config.badgeTextColor;
      this.ctx.textBaseline = "middle";
      this.ctx.fillText(badgeText, badgeX + 8, badgeY + badgeH / 2);
    }
  }
}
