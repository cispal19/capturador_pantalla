// @vitest-environment happy-dom
import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { RecordingToolbar } from "./RecordingToolbar";

describe("RecordingToolbar Component (Feature 007)", () => {
  let toolbar: RecordingToolbar;
  let container: HTMLElement;

  beforeEach(() => {
    vi.useFakeTimers();
    container = document.createElement("div");
    document.body.appendChild(container);
    toolbar = new RecordingToolbar();
    toolbar.mount(container);
  });

  afterEach(() => {
    toolbar.destroy();
    document.body.innerHTML = "";
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it("mounts hidden with initial timer at 00:00", () => {
    expect(toolbar.getElement().style.display).toBe("none");
    const timerEl = toolbar.getElement().querySelector('[data-role="timer"]');
    expect(timerEl?.textContent).toBe("00:00");
  });

  it("increments timer when active and formats as MM:SS", () => {
    toolbar.show();
    toolbar.startTimer();

    vi.advanceTimersByTime(3000);
    const timerEl = toolbar.getElement().querySelector('[data-role="timer"]');
    expect(timerEl?.textContent).toBe("00:03");
    expect(toolbar.getElapsedSeconds()).toBe(3);

    vi.advanceTimersByTime(62000); // +1 min 2 sec = 01:05
    expect(timerEl?.textContent).toBe("01:05");
    expect(toolbar.getElapsedSeconds()).toBe(65);
  });

  it("pauses and resumes timer correctly", () => {
    toolbar.show();
    toolbar.startTimer();

    vi.advanceTimersByTime(5000);
    expect(toolbar.getElapsedSeconds()).toBe(5);

    toolbar.pauseTimer();
    vi.advanceTimersByTime(5000); // Should not advance
    expect(toolbar.getElapsedSeconds()).toBe(5);

    toolbar.resumeTimer();
    vi.advanceTimersByTime(3000); // Should advance to 8
    expect(toolbar.getElapsedSeconds()).toBe(8);
  });

  it("dispatches callbacks on user interactions", () => {
    const pauseSpy = vi.fn();
    const stopSpy = vi.fn();
    const cancelSpy = vi.fn();

    toolbar.setOnPauseResume(pauseSpy);
    toolbar.setOnStop(stopSpy);
    toolbar.setOnCancel(cancelSpy);

    toolbar.show();
    toolbar.startTimer();

    const pauseBtn = toolbar
      .getElement()
      .querySelector<HTMLButtonElement>('[data-action="pause-resume"]')!;
    const stopBtn = toolbar.getElement().querySelector<HTMLButtonElement>('[data-action="stop"]')!;
    const cancelBtn = toolbar
      .getElement()
      .querySelector<HTMLButtonElement>('[data-action="cancel"]')!;

    pauseBtn.click();
    expect(pauseSpy).toHaveBeenCalledWith(true);

    pauseBtn.click();
    expect(pauseSpy).toHaveBeenCalledWith(false);

    stopBtn.click();
    expect(stopSpy).toHaveBeenCalledOnce();

    cancelBtn.click();
    expect(cancelSpy).toHaveBeenCalledOnce();
  });
});
