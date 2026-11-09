// @vitest-environment happy-dom
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { showToast } from "./Toast";

describe("Toast Component", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    document.body.innerHTML = "";
  });

  afterEach(() => {
    vi.useRealTimers();
    document.body.innerHTML = "";
  });

  it("renders toast message in the document body and removes after timeout", () => {
    const toast = showToast("Captura copiada al portapapeles", 2000);
    expect(toast).not.toBeNull();
    expect(document.body.contains(toast)).toBe(true);
    expect(toast.textContent).toBe("Captura copiada al portapapeles");

    // Fast forward
    vi.advanceTimersByTime(2250);
    expect(document.body.contains(toast)).toBe(false);
  });
});
