// @vitest-environment happy-dom
import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { SettingsModal } from "./SettingsModal";
import { tauriBridge } from "../services/tauriBridge";

describe("SettingsModal Component (Feature 006)", () => {
  let modal: SettingsModal;
  let container: HTMLElement;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
    modal = new SettingsModal();
    modal.mount(container);
  });

  afterEach(() => {
    modal.destroy();
    document.body.innerHTML = "";
    vi.restoreAllMocks();
  });

  it("mounts modal hidden by default", () => {
    expect(modal.isOpen()).toBe(false);
    expect(modal.getElement().style.display).toBe("none");
  });

  it("opens modal and loads current preferences into form fields", async () => {
    vi.spyOn(tauriBridge, "getPreferences").mockResolvedValue({
      capture_shortcut: "Alt+Shift+C",
      default_save_directory: "/Users/test/Capturas",
      default_image_format: "jpeg",
      autostart_enabled: true,
      show_notifications: true,
      copy_to_clipboard_on_capture: true,
    });

    await modal.open();

    expect(modal.isOpen()).toBe(true);
    expect(modal.getElement().style.display).not.toBe("none");

    const shortcutInput = modal.getElement().querySelector<HTMLInputElement>("#setting-shortcut");
    const formatSelect = modal.getElement().querySelector<HTMLSelectElement>("#setting-format");
    const dirInput = modal.getElement().querySelector<HTMLInputElement>("#setting-directory");
    const autostartCheck = modal.getElement().querySelector<HTMLInputElement>("#setting-autostart");

    expect(shortcutInput?.value).toBe("Alt+Shift+C");
    expect(formatSelect?.value).toBe("jpeg");
    expect(dirInput?.value).toBe("/Users/test/Capturas");
    expect(autostartCheck?.checked).toBe(true);
  });

  it("saves updated preferences via tauriBridge when clicking Guardar", async () => {
    await modal.open();

    const updateSpy = vi.spyOn(tauriBridge, "updatePreferences").mockResolvedValue({
      capture_shortcut: "Ctrl+Shift+S",
      default_save_directory: "/new/path",
      default_image_format: "jpeg",
      autostart_enabled: true,
      show_notifications: false,
      copy_to_clipboard_on_capture: false,
    });

    const shortcutInput = modal.getElement().querySelector<HTMLInputElement>("#setting-shortcut")!;
    shortcutInput.value = "Ctrl+Shift+S";

    const formatSelect = modal.getElement().querySelector<HTMLSelectElement>("#setting-format")!;
    formatSelect.value = "jpeg";

    const dirInput = modal.getElement().querySelector<HTMLInputElement>("#setting-directory")!;
    dirInput.value = "/new/path";

    const saveBtn = modal.getElement().querySelector<HTMLButtonElement>('[data-action="save"]')!;
    saveBtn.click();

    // Allow promise tick
    await new Promise((r) => setTimeout(r, 10));

    expect(updateSpy).toHaveBeenCalledOnce();
    expect(modal.isOpen()).toBe(false);
  });

  it("resets preferences to defaults when clicking Restablecer", async () => {
    await modal.open();

    const resetSpy = vi.spyOn(tauriBridge, "resetPreferences").mockResolvedValue({
      capture_shortcut: "CommandOrControl+Shift+X",
      default_save_directory: null,
      default_image_format: "png",
      autostart_enabled: false,
      show_notifications: true,
      copy_to_clipboard_on_capture: true,
    });

    const resetBtn = modal.getElement().querySelector<HTMLButtonElement>('[data-action="reset"]')!;
    resetBtn.click();

    await new Promise((r) => setTimeout(r, 10));

    expect(resetSpy).toHaveBeenCalledOnce();
    const formatSelect = modal.getElement().querySelector<HTMLSelectElement>("#setting-format")!;
    expect(formatSelect.value).toBe("png");
  });

  it("closes modal on cancel button or backdrop click", async () => {
    await modal.open();
    expect(modal.isOpen()).toBe(true);

    const cancelBtn = modal
      .getElement()
      .querySelector<HTMLButtonElement>('[data-action="cancel"]')!;
    cancelBtn.click();

    expect(modal.isOpen()).toBe(false);
  });
});
