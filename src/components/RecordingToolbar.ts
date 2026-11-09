import type { SelectionRect } from "../types/selection";

export class RecordingToolbar {
  private element: HTMLElement;
  private elapsedSeconds = 0;
  private isPaused = false;
  private timerInterval: ReturnType<typeof setInterval> | null = null;
  private onPauseResumeCallback?: (isPaused: boolean) => void;
  private onStopCallback?: () => void;
  private onCancelCallback?: () => void;

  constructor() {
    this.element = document.createElement("div");
    this.element.className = "recording-toolbar";
    this.element.style.position = "fixed";
    this.element.style.display = "none";
    this.element.style.zIndex = "1000001";
    this.element.style.background = "#0f172a";
    this.element.style.color = "#ffffff";
    this.element.style.border = "1px solid #334155";
    this.element.style.borderRadius = "8px";
    this.element.style.boxShadow =
      "0 10px 25px -5px rgba(0, 0, 0, 0.3), 0 8px 10px -6px rgba(0, 0, 0, 0.2)";
    this.element.style.padding = "6px 14px";
    this.element.style.gap = "10px";
    this.element.style.alignItems = "center";
    this.element.style.userSelect = "none";
    this.element.style.fontFamily =
      "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif";

    this.render();
  }

  mount(container: HTMLElement = document.body) {
    container.appendChild(this.element);
  }

  destroy() {
    this.resetTimer();
    if (this.element.parentNode) {
      this.element.parentNode.removeChild(this.element);
    }
  }

  getElement(): HTMLElement {
    return this.element;
  }

  show() {
    this.element.style.display = "flex";
  }

  hide() {
    this.element.style.display = "none";
  }

  setOnPauseResume(cb: (isPaused: boolean) => void) {
    this.onPauseResumeCallback = cb;
  }

  setOnStop(cb: () => void) {
    this.onStopCallback = cb;
  }

  setOnCancel(cb: () => void) {
    this.onCancelCallback = cb;
  }

  getElapsedSeconds(): number {
    return this.elapsedSeconds;
  }

  startTimer() {
    this.resetTimer();
    this.isPaused = false;
    this.updateDisplay();

    this.timerInterval = setInterval(() => {
      this.elapsedSeconds += 1;
      this.updateDisplay();
    }, 1000);
  }

  pauseTimer() {
    if (this.timerInterval !== null) {
      clearInterval(this.timerInterval);
      this.timerInterval = null;
    }
    this.isPaused = true;
    this.updateDisplay();
  }

  resumeTimer() {
    if (this.isPaused) {
      this.isPaused = false;
      this.updateDisplay();

      this.timerInterval = setInterval(() => {
        this.elapsedSeconds += 1;
        this.updateDisplay();
      }, 1000);
    }
  }

  resetTimer() {
    if (this.timerInterval !== null) {
      clearInterval(this.timerInterval);
      this.timerInterval = null;
    }
    this.elapsedSeconds = 0;
    this.isPaused = false;
    this.updateDisplay();
  }

  updatePosition(box: SelectionRect, viewportW: number, viewportH: number) {
    const margin = 10;
    const toolbarH = 44;
    const toolbarW = 280;

    // Default: position below the selection box
    let top = box.y + box.height + margin;
    let left = box.x + (box.width - toolbarW) / 2;

    // If bottom overflow, position above
    if (top + toolbarH > viewportH) {
      top = box.y - toolbarH - margin;
    }

    // Clamp inside viewport
    left = Math.max(margin, Math.min(left, viewportW - toolbarW - margin));
    top = Math.max(margin, Math.min(top, viewportH - toolbarH - margin));

    this.element.style.left = `${Math.round(left)}px`;
    this.element.style.top = `${Math.round(top)}px`;
  }

  private updateDisplay() {
    const timerEl = this.element.querySelector('[data-role="timer"]');
    const dotEl = this.element.querySelector<HTMLElement>('[data-role="indicator"]');
    const pauseBtn = this.element.querySelector<HTMLButtonElement>('[data-action="pause-resume"]');

    if (timerEl) {
      const mins = Math.floor(this.elapsedSeconds / 60);
      const secs = this.elapsedSeconds % 60;
      timerEl.textContent = `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
    }

    if (dotEl) {
      dotEl.style.backgroundColor = this.isPaused ? "#f59e0b" : "#ef4444";
    }

    if (pauseBtn) {
      pauseBtn.innerHTML = this.isPaused ? "▶️ Reanudar" : "⏸️ Pausar";
      pauseBtn.title = this.isPaused ? "Reanudar grabación" : "Pausar grabación";
    }
  }

  private render() {
    this.element.innerHTML = `
      <div style="display: flex; align-items: center; gap: 8px;">
        <span data-role="indicator" style="
          width: 10px; height: 10px; border-radius: 50%; background-color: #ef4444; display: inline-block;
        "></span>
        <span data-role="timer" style="
          font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
          font-size: 1rem; font-weight: 700; min-width: 48px;
        ">00:00</span>
      </div>

      <div style="display: flex; align-items: center; gap: 6px; margin-left: 6px;">
        <button data-action="pause-resume" style="
          background: #1e293b; color: #f8fafc; border: 1px solid #475569; border-radius: 6px; padding: 4px 10px; font-size: 0.8rem; font-weight: 500; cursor: pointer;
        ">⏸️ Pausar</button>
        <button data-action="stop" style="
          background: #dc2626; color: #ffffff; border: none; border-radius: 6px; padding: 4px 12px; font-size: 0.8rem; font-weight: 600; cursor: pointer;
        ">⏹️ Detener</button>
        <button data-action="cancel" style="
          background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 6px; padding: 4px 8px; font-size: 0.8rem; cursor: pointer;
        ">✕</button>
      </div>
    `;

    const pauseBtn = this.element.querySelector('[data-action="pause-resume"]');
    pauseBtn?.addEventListener("click", () => {
      if (this.isPaused) {
        this.resumeTimer();
        if (this.onPauseResumeCallback) this.onPauseResumeCallback(false);
      } else {
        this.pauseTimer();
        if (this.onPauseResumeCallback) this.onPauseResumeCallback(true);
      }
    });

    const stopBtn = this.element.querySelector('[data-action="stop"]');
    stopBtn?.addEventListener("click", () => {
      this.resetTimer();
      if (this.onStopCallback) this.onStopCallback();
    });

    const cancelBtn = this.element.querySelector('[data-action="cancel"]');
    cancelBtn?.addEventListener("click", () => {
      this.resetTimer();
      if (this.onCancelCallback) this.onCancelCallback();
    });
  }
}
