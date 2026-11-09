import { tauriBridge } from "../services/tauriBridge";
import { type AppPreferences, DEFAULT_PREFERENCES } from "../types/preferences";
import { showToast } from "./Toast";

export class SettingsModal {
  private element: HTMLElement;
  private currentPreferences: AppPreferences = { ...DEFAULT_PREFERENCES };
  private isModalOpen = false;

  constructor() {
    this.element = document.createElement("div");
    this.element.className = "settings-modal-backdrop";
    this.element.style.position = "fixed";
    this.element.style.top = "0";
    this.element.style.left = "0";
    this.element.style.width = "100vw";
    this.element.style.height = "100vh";
    this.element.style.backgroundColor = "rgba(15, 23, 42, 0.65)";
    this.element.style.backdropFilter = "blur(4px)";
    this.element.style.display = "none";
    this.element.style.zIndex = "2000000";
    this.element.style.alignItems = "center";
    this.element.style.justifyContent = "center";
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

  isOpen(): boolean {
    return this.isModalOpen;
  }

  async open(): Promise<void> {
    try {
      this.currentPreferences = await tauriBridge.getPreferences();
    } catch {
      this.currentPreferences = { ...DEFAULT_PREFERENCES };
    }

    this.populateForm();
    this.element.style.display = "flex";
    this.isModalOpen = true;
  }

  close(): void {
    this.element.style.display = "none";
    this.isModalOpen = false;
  }

  async save(): Promise<boolean> {
    const shortcutInput = this.element.querySelector<HTMLInputElement>("#setting-shortcut");
    const formatSelect = this.element.querySelector<HTMLSelectElement>("#setting-format");
    const dirInput = this.element.querySelector<HTMLInputElement>("#setting-directory");
    const clipboardCheck = this.element.querySelector<HTMLInputElement>("#setting-clipboard");
    const notificationsCheck =
      this.element.querySelector<HTMLInputElement>("#setting-notifications");
    const autostartCheck = this.element.querySelector<HTMLInputElement>("#setting-autostart");

    const shortcut = shortcutInput?.value.trim() || "";
    if (!shortcut) {
      showToast("⚠️ El atajo de captura no puede estar vacío");
      return false;
    }

    const updated: AppPreferences = {
      capture_shortcut: shortcut,
      default_image_format: (formatSelect?.value as "png" | "jpeg") || "png",
      default_save_directory: dirInput?.value.trim() || null,
      copy_to_clipboard_on_capture: clipboardCheck?.checked ?? true,
      show_notifications: notificationsCheck?.checked ?? true,
      autostart_enabled: autostartCheck?.checked ?? false,
    };

    try {
      this.currentPreferences = await tauriBridge.updatePreferences(updated);
      showToast("✅ Preferencias guardadas correctamente");
      this.close();
      return true;
    } catch {
      showToast("⚠️ Error al guardar preferencias");
      return false;
    }
  }

  async reset(): Promise<void> {
    try {
      this.currentPreferences = await tauriBridge.resetPreferences();
      this.populateForm();
      showToast("🔄 Valores por defecto restaurados");
    } catch {
      showToast("⚠️ Error al restaurar valores por defecto");
    }
  }

  private populateForm(): void {
    const shortcutInput = this.element.querySelector<HTMLInputElement>("#setting-shortcut");
    const formatSelect = this.element.querySelector<HTMLSelectElement>("#setting-format");
    const dirInput = this.element.querySelector<HTMLInputElement>("#setting-directory");
    const clipboardCheck = this.element.querySelector<HTMLInputElement>("#setting-clipboard");
    const notificationsCheck =
      this.element.querySelector<HTMLInputElement>("#setting-notifications");
    const autostartCheck = this.element.querySelector<HTMLInputElement>("#setting-autostart");

    if (shortcutInput) shortcutInput.value = this.currentPreferences.capture_shortcut;
    if (formatSelect) formatSelect.value = this.currentPreferences.default_image_format;
    if (dirInput) dirInput.value = this.currentPreferences.default_save_directory || "";
    if (clipboardCheck)
      clipboardCheck.checked = this.currentPreferences.copy_to_clipboard_on_capture;
    if (notificationsCheck) notificationsCheck.checked = this.currentPreferences.show_notifications;
    if (autostartCheck) autostartCheck.checked = this.currentPreferences.autostart_enabled;
  }

  private render(): void {
    this.element.innerHTML = `
      <div class="settings-card" style="
        background: #ffffff;
        border-radius: 12px;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.2), 0 10px 10px -5px rgba(0, 0, 0, 0.1);
        width: 100%;
        max-width: 480px;
        padding: 24px;
        color: #1e293b;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        box-sizing: border-box;
      ">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; border-bottom: 1px solid #e2e8f0; padding-bottom: 12px;">
          <h2 style="margin: 0; font-size: 1.25rem; font-weight: 600; display: flex; align-items: center; gap: 8px;">
            <span>⚙️</span> Preferencias de Captura
          </h2>
          <button data-action="close" style="
            background: none; border: none; font-size: 1.25rem; cursor: pointer; color: #64748b; padding: 4px; border-radius: 4px;
          ">✕</button>
        </div>

        <div style="display: flex; flex-direction: column; gap: 16px;">
          <!-- Atajo Global -->
          <div>
            <label style="display: block; font-size: 0.875rem; font-weight: 600; margin-bottom: 6px; color: #334155;">
              Atajo Global de Activación
            </label>
            <input id="setting-shortcut" type="text" placeholder="CommandOrControl+Shift+X" style="
              width: 100%; padding: 8px 12px; border: 1px solid #cbd5e1; border-radius: 6px; font-size: 0.875rem; box-sizing: border-box; outline: none;
            " />
            <small style="color: #64748b; font-size: 0.75rem;">Ej: CommandOrControl+Shift+X, Alt+Shift+C o PrintScreen</small>
          </div>

          <!-- Formato de Guardado -->
          <div>
            <label style="display: block; font-size: 0.875rem; font-weight: 600; margin-bottom: 6px; color: #334155;">
              Formato de Imagen Predeterminado
            </label>
            <select id="setting-format" style="
              width: 100%; padding: 8px 12px; border: 1px solid #cbd5e1; border-radius: 6px; font-size: 0.875rem; box-sizing: border-box; background: #fff;
            ">
              <option value="png">PNG (Sin pérdidas, recomendado para texto y diagramas)</option>
              <option value="jpeg">JPEG (Comprimido, menor peso de archivo)</option>
            </select>
          </div>

          <!-- Carpeta de Guardado -->
          <div>
            <label style="display: block; font-size: 0.875rem; font-weight: 600; margin-bottom: 6px; color: #334155;">
              Carpeta de Guardado Local
            </label>
            <input id="setting-directory" type="text" placeholder="Carpeta predeterminada de Imágenes del SO" style="
              width: 100%; padding: 8px 12px; border: 1px solid #cbd5e1; border-radius: 6px; font-size: 0.875rem; box-sizing: border-box; outline: none;
            " />
            <small style="color: #64748b; font-size: 0.75rem;">Dejar vacío para usar la carpeta estándar de Imágenes del usuario</small>
          </div>

          <!-- Casillas de verificación -->
          <div style="display: flex; flex-direction: column; gap: 8px; margin-top: 4px;">
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.875rem; cursor: pointer;">
              <input id="setting-clipboard" type="checkbox" />
              <span>Copiar automáticamente al portapapeles tras capturar</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.875rem; cursor: pointer;">
              <input id="setting-notifications" type="checkbox" />
              <span>Mostrar notificaciones visuales toast de confirmación</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.875rem; cursor: pointer;">
              <input id="setting-autostart" type="checkbox" />
              <span>Iniciar automáticamente con el sistema operativo</span>
            </label>
          </div>
        </div>

        <!-- Botones de Acción -->
        <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 24px; border-top: 1px solid #e2e8f0; padding-top: 16px;">
          <button data-action="reset" style="
            background: #f1f5f9; color: #475569; border: 1px solid #cbd5e1; border-radius: 6px; padding: 8px 14px; font-size: 0.875rem; font-weight: 500; cursor: pointer;
          ">Restablecer</button>
          <div style="display: flex; gap: 8px;">
            <button data-action="cancel" style="
              background: #fff; color: #64748b; border: 1px solid #cbd5e1; border-radius: 6px; padding: 8px 14px; font-size: 0.875rem; font-weight: 500; cursor: pointer;
            ">Cancelar</button>
            <button data-action="save" style="
              background: #2563eb; color: #fff; border: none; border-radius: 6px; padding: 8px 16px; font-size: 0.875rem; font-weight: 500; cursor: pointer; box-shadow: 0 1px 2px rgba(0,0,0,0.05);
            ">Guardar Preferencias</button>
          </div>
        </div>
      </div>
    `;

    // Event Listeners
    this.element.addEventListener("click", (e) => {
      if (e.target === this.element) {
        this.close();
      }
    });

    const closeBtn = this.element.querySelector('[data-action="close"]');
    closeBtn?.addEventListener("click", () => this.close());

    const cancelBtn = this.element.querySelector('[data-action="cancel"]');
    cancelBtn?.addEventListener("click", () => this.close());

    const saveBtn = this.element.querySelector('[data-action="save"]');
    saveBtn?.addEventListener("click", () => void this.save());

    const resetBtn = this.element.querySelector('[data-action="reset"]');
    resetBtn?.addEventListener("click", () => void this.reset());
  }
}
