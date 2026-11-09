import {
  DEFAULT_OVERLAY_CONFIG,
  type ResizeHandleType,
  type SelectionInteractionState,
  type SelectionOverlayConfig,
  type SelectionRect,
} from "../types/selection";
import type { AnnotationItem } from "../types/annotations";
import {
  getHandleAnchors,
  hitTestHandle,
  normalizeSelection,
  pointInRect,
  resizeByHandle,
  translateRect,
} from "../domain/geometry";
import { UndoRedoStack, createAnnotation } from "../domain/annotations";
import { AnnotationToolbar } from "./AnnotationToolbar";
import { tauriBridge } from "../services/tauriBridge";

export class SelectionOverlay {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D | null = null;
  private config: SelectionOverlayConfig;
  private state: SelectionInteractionState = { mode: "idle" };
  private activeBox: SelectionRect | null = null;
  private container: HTMLElement | null = null;
  private animationFrameId: number | null = null;
  private toolbar: AnnotationToolbar;
  private history = new UndoRedoStack<AnnotationItem>();
  private activeDrawing: AnnotationItem | null = null;
  private drawingPoints: Array<[number, number]> = [];
  private drawStartX = 0;
  private drawStartY = 0;
  private isDrawingAnnotation = false;

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

    this.toolbar = new AnnotationToolbar();

    this.handleMouseDown = this.handleMouseDown.bind(this);
    this.handleMouseMove = this.handleMouseMove.bind(this);
    this.handleMouseUp = this.handleMouseUp.bind(this);
    this.handleKeyDown = this.handleKeyDown.bind(this);
    this.handleResize = this.handleResize.bind(this);
    this.render = this.render.bind(this);

    this.toolbar.setOnUndo(() => this.undo());
    this.toolbar.setOnRedo(() => this.redo());
    this.toolbar.setOnToolChange(() => this.updateCursor());
  }

  mount(container: HTMLElement = document.body) {
    this.container = container;
    this.container.appendChild(this.canvas);
    this.toolbar.mount(container);
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

    this.toolbar.destroy();

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

  getToolbar(): AnnotationToolbar {
    return this.toolbar;
  }

  getAnnotations(): AnnotationItem[] {
    return this.history.getItems();
  }

  undo() {
    this.history.undo();
    this.requestRender();
    void tauriBridge.syncAnnotations(this.history.getItems());
  }

  redo() {
    this.history.redo();
    this.requestRender();
    void tauriBridge.syncAnnotations(this.history.getItems());
  }

  cancel() {
    this.activeBox = null;
    this.state = { mode: "idle" };
    this.toolbar.hide();
    this.history.clear();
    this.canvas.style.cursor = "crosshair";
    this.requestRender();

    void tauriBridge.cancelCaptureSession();

    if (this.onCancelCallback) {
      this.onCancelCallback();
    }
  }

  private updateCursor() {
    if (!this.activeBox) {
      this.canvas.style.cursor = "crosshair";
      return;
    }

    const tool = this.toolbar.getActiveTool();
    if (tool === "select") {
      this.canvas.style.cursor = "default";
    } else if (tool === "text") {
      this.canvas.style.cursor = "text";
    } else {
      this.canvas.style.cursor = "crosshair";
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
    if (this.activeBox) {
      this.toolbar.updatePosition(
        this.activeBox,
        window.innerWidth || 800,
        window.innerHeight || 600
      );
    }
    this.requestRender();
  }

  private handleMouseDown(e: MouseEvent) {
    const x = e.clientX;
    const y = e.clientY;
    const tool = this.toolbar.getActiveTool();

    if (this.activeBox) {
      // If an annotation tool is active, start drawing annotation inside selection box
      if (tool !== "select" && pointInRect(this.activeBox, x, y)) {
        this.isDrawingAnnotation = true;
        this.drawStartX = x;
        this.drawStartY = y;
        this.drawingPoints = [[x, y]];

        const color = this.toolbar.getActiveColor();
        const thickness = this.toolbar.getActiveThickness();

        if (tool === "text") {
          const content = window.prompt("Ingrese el texto para la anotación:", "Nota");
          if (content && content.trim().length > 0) {
            const anno = createAnnotation.text(x, y, content.trim(), 16, color);
            this.history.push(anno);
            void tauriBridge.syncAnnotations(this.history.getItems());
            this.requestRender();
          }
          this.isDrawingAnnotation = false;
          return;
        }

        if (tool === "rectangle") {
          this.activeDrawing = createAnnotation.rectangle(x, y, 0, 0, color, thickness);
        } else if (tool === "ellipse") {
          this.activeDrawing = createAnnotation.ellipse(x, y, 0, 0, color, thickness);
        } else if (tool === "arrow") {
          this.activeDrawing = createAnnotation.arrow(x, y, x, y, color, thickness);
        } else if (tool === "line") {
          this.activeDrawing = createAnnotation.line(x, y, x, y, color, thickness);
        } else if (tool === "freehand") {
          this.activeDrawing = createAnnotation.freehand(this.drawingPoints, color, thickness);
        } else if (tool === "highlighter") {
          this.activeDrawing = createAnnotation.highlighter(
            this.drawingPoints,
            color,
            thickness * 3
          );
        } else if (tool === "obfuscation") {
          this.activeDrawing = createAnnotation.obfuscation(x, y, 0, 0, 12);
        }

        this.requestRender();
        return;
      }

      // Check if clicking on an interactive handle
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
        this.toolbar.hide();
        this.requestRender();
        return;
      }

      // Check if clicking inside selection box (translate)
      if (pointInRect(this.activeBox, x, y)) {
        this.state = {
          mode: "moving",
          initialBox: { ...this.activeBox },
          startX: x,
          startY: y,
          currentX: x,
          currentY: y,
        };
        this.toolbar.hide();
        this.canvas.style.cursor = "move";
        this.requestRender();
        return;
      }
    }

    // Start a new selection drag
    this.activeBox = null;
    this.toolbar.hide();
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

    if (this.isDrawingAnnotation && this.activeDrawing) {
      const tool = this.toolbar.getActiveTool();
      const color = this.toolbar.getActiveColor();
      const thickness = this.toolbar.getActiveThickness();

      if (tool === "rectangle") {
        const norm = normalizeSelection(this.drawStartX, this.drawStartY, x, y);
        this.activeDrawing = createAnnotation.rectangle(
          norm.x,
          norm.y,
          norm.width,
          norm.height,
          color,
          thickness
        );
      } else if (tool === "ellipse") {
        const rx = Math.abs(x - this.drawStartX) / 2;
        const ry = Math.abs(y - this.drawStartY) / 2;
        const cx = Math.min(this.drawStartX, x) + rx;
        const cy = Math.min(this.drawStartY, y) + ry;
        this.activeDrawing = createAnnotation.ellipse(cx, cy, rx, ry, color, thickness);
      } else if (tool === "arrow") {
        this.activeDrawing = createAnnotation.arrow(
          this.drawStartX,
          this.drawStartY,
          x,
          y,
          color,
          thickness
        );
      } else if (tool === "line") {
        this.activeDrawing = createAnnotation.line(
          this.drawStartX,
          this.drawStartY,
          x,
          y,
          color,
          thickness
        );
      } else if (tool === "freehand" || tool === "highlighter") {
        this.drawingPoints.push([x, y]);
        if (tool === "freehand") {
          this.activeDrawing = createAnnotation.freehand([...this.drawingPoints], color, thickness);
        } else {
          this.activeDrawing = createAnnotation.highlighter(
            [...this.drawingPoints],
            color,
            thickness * 3
          );
        }
      } else if (tool === "obfuscation") {
        const norm = normalizeSelection(this.drawStartX, this.drawStartY, x, y);
        this.activeDrawing = createAnnotation.obfuscation(
          norm.x,
          norm.y,
          norm.width,
          norm.height,
          12
        );
      }

      this.requestRender();
      return;
    }

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
        if (this.activeBox && this.toolbar.getActiveTool() === "select") {
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
    if (this.isDrawingAnnotation && this.activeDrawing) {
      this.history.push(this.activeDrawing);
      void tauriBridge.syncAnnotations(this.history.getItems());

      if (this.activeDrawing.type === "obfuscation") {
        void tauriBridge.applyObfuscation(
          this.activeDrawing.x,
          this.activeDrawing.y,
          this.activeDrawing.width,
          this.activeDrawing.height,
          this.activeDrawing.block_size
        );
      }

      this.activeDrawing = null;
      this.isDrawingAnnotation = false;
      this.requestRender();
      return;
    }

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
        this.toolbar.updatePosition(
          this.activeBox,
          typeof window !== "undefined" ? window.innerWidth || 800 : 800,
          typeof window !== "undefined" ? window.innerHeight || 600 : 600
        );
        this.updateCursor();
        if (this.onSelectionCompleteCallback) {
          this.onSelectionCompleteCallback(this.activeBox);
        }
      } else {
        this.activeBox = null;
        this.state = { mode: "idle" };
        this.toolbar.hide();
        this.canvas.style.cursor = "crosshair";
      }
      this.requestRender();
    }
  }

  private handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      this.cancel();
    } else if ((e.ctrlKey || e.metaKey) && (e.key === "z" || e.key === "Z") && !e.shiftKey) {
      e.preventDefault();
      this.undo();
    } else if (
      ((e.ctrlKey || e.metaKey) && (e.key === "y" || e.key === "Y")) ||
      ((e.ctrlKey || e.metaKey) && e.shiftKey && (e.key === "z" || e.key === "Z"))
    ) {
      e.preventDefault();
      this.redo();
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

    // 1. Draw 40% dimming
    this.ctx.fillStyle = `rgba(0, 0, 0, ${this.config.dimOpacity})`;
    this.ctx.fillRect(0, 0, width, height);

    if (this.activeBox && this.activeBox.width > 0 && this.activeBox.height > 0) {
      const { x, y, width: bw, height: bh } = this.activeBox;

      // 2. Clear out selection rectangle
      this.ctx.clearRect(x, y, bw, bh);

      // 3. Render committed annotations + active preview
      this.ctx.save();
      this.ctx.beginPath();
      this.ctx.rect(x, y, bw, bh);
      this.ctx.clip();

      for (const item of this.history.getItems()) {
        this.renderAnnotationItem(item);
      }
      if (this.activeDrawing) {
        this.renderAnnotationItem(this.activeDrawing);
      }
      this.ctx.restore();

      // 4. Draw bounding box border
      this.ctx.strokeStyle = this.config.borderColor;
      this.ctx.lineWidth = this.config.borderWidth;
      this.ctx.strokeRect(x, y, bw, bh);

      // 5. Draw 8 resize handles
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

      // 6. Draw dimensions badge
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

  private renderAnnotationItem(item: AnnotationItem) {
    if (!this.ctx) return;

    this.ctx.save();

    switch (item.type) {
      case "rectangle": {
        this.ctx.strokeStyle = item.color;
        this.ctx.lineWidth = item.stroke_width;
        this.ctx.strokeRect(item.x, item.y, item.width, item.height);
        break;
      }
      case "ellipse": {
        this.ctx.strokeStyle = item.color;
        this.ctx.lineWidth = item.stroke_width;
        this.ctx.beginPath();
        this.ctx.ellipse(
          item.x,
          item.y,
          Math.max(1, item.radius_x),
          Math.max(1, item.radius_y),
          0,
          0,
          Math.PI * 2
        );
        this.ctx.stroke();
        break;
      }
      case "line": {
        this.ctx.strokeStyle = item.color;
        this.ctx.lineWidth = item.stroke_width;
        this.ctx.beginPath();
        this.ctx.moveTo(item.start_x, item.start_y);
        this.ctx.lineTo(item.end_x, item.end_y);
        this.ctx.stroke();
        break;
      }
      case "arrow": {
        this.ctx.strokeStyle = item.color;
        this.ctx.fillStyle = item.color;
        this.ctx.lineWidth = item.stroke_width;

        const dx = item.end_x - item.start_x;
        const dy = item.end_y - item.start_y;
        const angle = Math.atan2(dy, dx);
        const headLen = Math.max(10, item.stroke_width * 3);

        this.ctx.beginPath();
        this.ctx.moveTo(item.start_x, item.start_y);
        this.ctx.lineTo(item.end_x, item.end_y);
        this.ctx.stroke();

        this.ctx.beginPath();
        this.ctx.moveTo(item.end_x, item.end_y);
        this.ctx.lineTo(
          item.end_x - headLen * Math.cos(angle - Math.PI / 6),
          item.end_y - headLen * Math.sin(angle - Math.PI / 6)
        );
        this.ctx.lineTo(
          item.end_x - headLen * Math.cos(angle + Math.PI / 6),
          item.end_y - headLen * Math.sin(angle + Math.PI / 6)
        );
        this.ctx.closePath();
        this.ctx.fill();
        break;
      }
      case "freehand": {
        if (item.points.length < 2) break;
        this.ctx.strokeStyle = item.color;
        this.ctx.lineWidth = item.stroke_width;
        this.ctx.lineCap = "round";
        this.ctx.lineJoin = "round";
        this.ctx.beginPath();
        this.ctx.moveTo(item.points[0][0], item.points[0][1]);
        for (let i = 1; i < item.points.length; i++) {
          this.ctx.lineTo(item.points[i][0], item.points[i][1]);
        }
        this.ctx.stroke();
        break;
      }
      case "highlighter": {
        if (item.points.length < 2) break;
        this.ctx.strokeStyle = item.color;
        this.ctx.lineWidth = item.stroke_width;
        this.ctx.globalAlpha = 0.35;
        this.ctx.lineCap = "round";
        this.ctx.lineJoin = "round";
        this.ctx.beginPath();
        this.ctx.moveTo(item.points[0][0], item.points[0][1]);
        for (let i = 1; i < item.points.length; i++) {
          this.ctx.lineTo(item.points[i][0], item.points[i][1]);
        }
        this.ctx.stroke();
        break;
      }
      case "text": {
        this.ctx.fillStyle = item.color;
        this.ctx.font = `bold ${item.font_size}px system-ui, sans-serif`;
        this.ctx.textBaseline = "top";
        this.ctx.fillText(item.content, item.x, item.y);
        break;
      }
      case "obfuscation": {
        // Draw checkered / pixelated blur mask
        const bs = item.block_size;
        for (let py = item.y; py < item.y + item.height; py += bs) {
          for (let px = item.x; px < item.x + item.width; px += bs) {
            const isDark = (Math.floor(px / bs) + Math.floor(py / bs)) % 2 === 0;
            this.ctx.fillStyle = isDark ? "#94a3b8" : "#cbd5e1";
            this.ctx.fillRect(
              px,
              py,
              Math.min(bs, item.x + item.width - px),
              Math.min(bs, item.y + item.height - py)
            );
          }
        }
        this.ctx.strokeStyle = "#64748b";
        this.ctx.lineWidth = 1;
        this.ctx.strokeRect(item.x, item.y, item.width, item.height);
        break;
      }
    }

    this.ctx.restore();
  }
}
