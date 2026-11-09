import {
  CORPORATE_PALETTE,
  type ActiveTool,
  type CorporateColor,
  type StrokeThickness,
} from "../types/annotations";
import type { SelectionRect } from "../types/selection";

export class AnnotationToolbar {
  private element: HTMLElement;
  private activeTool: ActiveTool = "select";
  private activeColor: CorporateColor = "#ef4444";
  private activeThickness: StrokeThickness = 4;
  private onToolChangeCallback?: (tool: ActiveTool) => void;
  private onColorChangeCallback?: (color: CorporateColor) => void;
  private onThicknessChangeCallback?: (thickness: StrokeThickness) => void;
  private onUndoCallback?: () => void;
  private onRedoCallback?: () => void;
  private onCopyCallback?: () => void;
  private onSaveCallback?: () => void;
  private onRecordCallback?: () => void;

  constructor() {
    this.element = document.createElement("div");
    this.element.className = "annotation-toolbar";
    this.element.style.position = "fixed";
    this.element.style.display = "none";
    this.element.style.zIndex = "1000000";
    this.element.style.background = "#ffffff";
    this.element.style.border = "1px solid #cbd5e1";
    this.element.style.borderRadius = "8px";
    this.element.style.boxShadow =
      "0 10px 25px -5px rgba(0, 0, 0, 0.15), 0 8px 10px -6px rgba(0, 0, 0, 0.1)";
    this.element.style.padding = "6px 10px";
    this.element.style.gap = "6px";
    this.element.style.alignItems = "center";
    this.element.style.userSelect = "none";

    this.render();
  }

  mount(container: HTMLElement = document.body) {
    container.appendChild(this.element);
  }

  destroy() {
    if (this.element.parentNode) {
      this.element.parentNode.removeChild(this.element);
    }
  }

  getElement(): HTMLElement {
    return this.element;
  }

  getActiveTool(): ActiveTool {
    return this.activeTool;
  }

  getActiveColor(): CorporateColor {
    return this.activeColor;
  }

  getActiveThickness(): StrokeThickness {
    return this.activeThickness;
  }

  setOnToolChange(cb: (tool: ActiveTool) => void) {
    this.onToolChangeCallback = cb;
  }

  setOnColorChange(cb: (color: CorporateColor) => void) {
    this.onColorChangeCallback = cb;
  }

  setOnThicknessChange(cb: (thickness: StrokeThickness) => void) {
    this.onThicknessChangeCallback = cb;
  }

  setOnUndo(cb: () => void) {
    this.onUndoCallback = cb;
  }

  setOnRedo(cb: () => void) {
    this.onRedoCallback = cb;
  }

  setOnCopy(cb: () => void) {
    this.onCopyCallback = cb;
  }

  setOnSave(cb: () => void) {
    this.onSaveCallback = cb;
  }

  setOnRecord(cb: () => void) {
    this.onRecordCallback = cb;
  }

  show() {
    this.element.style.display = "flex";
  }

  hide() {
    this.element.style.display = "none";
  }

  updatePosition(box: SelectionRect, viewportW: number, viewportH: number) {
    this.show();
    const toolbarH = 46;
    const toolbarW = 620;

    let top = box.y + box.height + 8;
    // If overflows bottom margin, place on top
    if (top + toolbarH > viewportH - 8) {
      top = box.y - toolbarH - 8;
      // If overflows top too, place inside bottom
      if (top < 8) {
        top = box.y + box.height - toolbarH - 8;
      }
    }

    // Align right with the selection box
    let left = box.x + box.width - toolbarW;
    if (left < 8) {
      left = 8;
    }
    if (left + toolbarW > viewportW - 8) {
      left = viewportW - toolbarW - 8;
    }

    this.element.style.top = `${Math.round(top)}px`;
    this.element.style.left = `${Math.round(left)}px`;
  }

  private render() {
    this.element.innerHTML = `
      <div style="display: flex; gap: 4px; align-items: center;">
        <button type="button" data-tool="select" title="Seleccionar / Mover" style="${this.btnStyle("select")}">🖱️</button>
        <button type="button" data-tool="rectangle" title="Rectángulo (R)" style="${this.btnStyle("rectangle")}">🔲</button>
        <button type="button" data-tool="ellipse" title="Elipse" style="${this.btnStyle("ellipse")}">⭕</button>
        <button type="button" data-tool="arrow" title="Flecha (A)" style="${this.btnStyle("arrow")}">↗️</button>
        <button type="button" data-tool="line" title="Línea recta" style="${this.btnStyle("line")}">➖</button>
        <button type="button" data-tool="freehand" title="Lápiz a mano alzada" style="${this.btnStyle("freehand")}">✏️</button>
        <button type="button" data-tool="text" title="Texto tipográfico (T)" style="${this.btnStyle("text")}">🔤</button>
        <button type="button" data-tool="highlighter" title="Resaltador" style="${this.btnStyle("highlighter")}">🖍️</button>
        <button type="button" data-tool="obfuscation" title="Ofuscar / Pixelar datos (B)" style="${this.btnStyle("obfuscation")}">🛡️</button>
      </div>

      <div style="width: 1px; height: 24px; background: #e2e8f0; margin: 0 4px;"></div>

      <div style="display: flex; gap: 4px; align-items: center;">
        ${CORPORATE_PALETTE.map(
          (color) => `
          <button type="button" data-color="${color}" title="${color}" style="
            width: 18px;
            height: 18px;
            border-radius: 50%;
            background: ${color};
            border: 2px solid ${this.activeColor === color ? "#0f172a" : "#cbd5e1"};
            cursor: pointer;
            padding: 0;
          "></button>
        `
        ).join("")}
      </div>

      <div style="width: 1px; height: 24px; background: #e2e8f0; margin: 0 4px;"></div>

      <div style="display: flex; gap: 4px; align-items: center;">
        <button type="button" data-thickness="2" title="Grosor fino" style="${this.thickStyle(2)}">1x</button>
        <button type="button" data-thickness="4" title="Grosor medio" style="${this.thickStyle(4)}">2x</button>
        <button type="button" data-thickness="8" title="Grosor grueso" style="${this.thickStyle(8)}">3x</button>
      </div>

      <div style="width: 1px; height: 24px; background: #e2e8f0; margin: 0 4px;"></div>

      <div style="display: flex; gap: 4px; align-items: center;">
        <button type="button" data-action="undo" title="Deshacer (Ctrl+Z)" style="${this.btnStyle("select")}">↩️</button>
        <button type="button" data-action="redo" title="Rehacer (Ctrl+Y)" style="${this.btnStyle("select")}">↪️</button>
      </div>

      <div style="width: 1px; height: 24px; background: #e2e8f0; margin: 0 4px;"></div>

      <div style="display: flex; gap: 4px; align-items: center;">
        <button type="button" data-action="copy" title="Copiar al portapapeles (Enter)" style="
          background: #2563eb;
          color: #ffffff;
          border: none;
          border-radius: 6px;
          padding: 4px 10px;
          font-size: 13px;
          font-weight: 600;
          cursor: pointer;
          display: flex;
          align-items: center;
          gap: 4px;
        ">📋 Copiar</button>
        <button type="button" data-action="save" title="Guardar en disco (Cmd+S)" style="
          background: #f1f5f9;
          color: #0f172a;
          border: 1px solid #cbd5e1;
          border-radius: 6px;
          padding: 4px 10px;
          font-size: 13px;
          font-weight: 500;
          cursor: pointer;
          display: flex;
          align-items: center;
          gap: 4px;
        ">💾 Guardar</button>
        <button type="button" data-action="record" title="Grabar video de la región seleccionada" style="
          background: #dc2626;
          color: #ffffff;
          border: none;
          border-radius: 6px;
          padding: 4px 10px;
          font-size: 13px;
          font-weight: 600;
          cursor: pointer;
          display: flex;
          align-items: center;
          gap: 4px;
        ">🎥 Grabar</button>
      </div>
    `;

    this.attachEvents();
  }

  private btnStyle(tool: ActiveTool): string {
    const isSelected = this.activeTool === tool;
    return `
      background: ${isSelected ? "#e0f2fe" : "transparent"};
      border: 1px solid ${isSelected ? "#0284c7" : "transparent"};
      border-radius: 6px;
      padding: 4px 6px;
      font-size: 14px;
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
    `;
  }

  private thickStyle(size: StrokeThickness): string {
    const isSelected = this.activeThickness === size;
    return `
      background: ${isSelected ? "#e0f2fe" : "#f8fafc"};
      border: 1px solid ${isSelected ? "#0284c7" : "#cbd5e1"};
      border-radius: 4px;
      padding: 2px 5px;
      font-size: 11px;
      font-weight: 600;
      cursor: pointer;
    `;
  }

  private attachEvents() {
    this.element.querySelectorAll<HTMLButtonElement>("[data-tool]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const tool = btn.getAttribute("data-tool") as ActiveTool;
        this.activeTool = tool;
        this.render();
        if (this.onToolChangeCallback) {
          this.onToolChangeCallback(tool);
        }
      });
    });

    this.element.querySelectorAll<HTMLButtonElement>("[data-color]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const color = btn.getAttribute("data-color") as CorporateColor;
        this.activeColor = color;
        this.render();
        if (this.onColorChangeCallback) {
          this.onColorChangeCallback(color);
        }
      });
    });

    this.element.querySelectorAll<HTMLButtonElement>("[data-thickness]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const thickness = parseInt(
          btn.getAttribute("data-thickness") || "4",
          10
        ) as StrokeThickness;
        this.activeThickness = thickness;
        this.render();
        if (this.onThicknessChangeCallback) {
          this.onThicknessChangeCallback(thickness);
        }
      });
    });

    const undoBtn = this.element.querySelector('[data-action="undo"]');
    if (undoBtn) {
      undoBtn.addEventListener("click", () => {
        if (this.onUndoCallback) this.onUndoCallback();
      });
    }

    const redoBtn = this.element.querySelector('[data-action="redo"]');
    if (redoBtn) {
      redoBtn.addEventListener("click", () => {
        if (this.onRedoCallback) this.onRedoCallback();
      });
    }

    const copyBtn = this.element.querySelector('[data-action="copy"]');
    if (copyBtn) {
      copyBtn.addEventListener("click", () => {
        if (this.onCopyCallback) this.onCopyCallback();
      });
    }

    const saveBtn = this.element.querySelector('[data-action="save"]');
    if (saveBtn) {
      saveBtn.addEventListener("click", () => {
        if (this.onSaveCallback) this.onSaveCallback();
      });
    }

    const recordBtn = this.element.querySelector('[data-action="record"]');
    if (recordBtn) {
      recordBtn.addEventListener("click", () => {
        if (this.onRecordCallback) this.onRecordCallback();
      });
    }
  }
}
