import "./App.css";
import { tauriBridge } from "./services/tauriBridge";
import { SelectionOverlay } from "./components/SelectionOverlay";
import { SettingsModal } from "./components/SettingsModal";
import { listen } from "@tauri-apps/api/event";

let activeOverlay: SelectionOverlay | null = null;
let settingsModal: SettingsModal | null = null;

async function initOverlay() {
  document.documentElement.classList.add("overlay-mode");
  document.body.classList.add("overlay-mode");
  const root = document.getElementById("root");
  if (root) root.remove();

  let overlayInstance: SelectionOverlay | null = null;

  try {
    await listen<{ imageUrl?: string }>("start-capture", async (event) => {
      if (overlayInstance) {
        overlayInstance.destroy();
        overlayInstance = null;
      }

      let preloadedImg: HTMLImageElement | null = null;
      if (event.payload?.imageUrl) {
        preloadedImg = new Image();
        await new Promise<void>((resolve) => {
          preloadedImg!.onload = () => resolve();
          preloadedImg!.onerror = () => resolve();
          preloadedImg!.src = event.payload.imageUrl!;
        });
      }

      overlayInstance = new SelectionOverlay({
        preloadedImage: preloadedImg,
        backgroundImageUrl: event.payload?.imageUrl,
      });
      overlayInstance.mount();

      overlayInstance.setOnCancel(async () => {
        if (overlayInstance) {
          overlayInstance.destroy();
          overlayInstance = null;
        }
        await tauriBridge.hideCaptureOverlay();
      });
    });
  } catch {}
}

async function startSelection() {
  // Close preferences modal if open
  if (settingsModal?.isOpen()) {
    settingsModal.close();
  }

  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    if (getCurrentWindow()) {
      await tauriBridge.startCapture();
      return;
    }
  } catch {}

  // Fallback for headless / non-Tauri test environments
  const captureResult = await tauriBridge.triggerScreenCapture();
  if (activeOverlay) {
    activeOverlay.destroy();
    activeOverlay = null;
  }
  activeOverlay = new SelectionOverlay({
    backgroundImageUrl: captureResult.image_data_url,
  });
  activeOverlay.mount();
  activeOverlay.setOnCancel(() => {
    if (activeOverlay) {
      activeOverlay.destroy();
      activeOverlay = null;
    }
  });
}

async function renderApp() {
  const root = document.getElementById("root");
  if (!root) return;

  const appInfo = await tauriBridge.getAppInfo();
  const monitors = await tauriBridge.getConnectedMonitors();
  const primary = monitors.find((m) => m.is_primary) ?? monitors[0];

  settingsModal = new SettingsModal();
  settingsModal.mount();

  root.innerHTML = `
    <div class="container">
      <h1>Capturador de Pantalla</h1>
      <span class="badge-local-first">🛡️ 100% Local-First / Zero Network</span>
      <p>Herramienta corporativa de captura de pantalla y anotaciones de alta fidelidad.</p>
      
      <div style="margin: 1.5rem 0; display: flex; gap: 12px; justify-content: center; flex-wrap: wrap;">
        <button id="btn-capture" style="background: #2563eb; color: #ffffff; border: none; padding: 0.75rem 1.5rem; font-size: 1rem; font-weight: 600; border-radius: 8px; cursor: pointer; transition: background 0.2s;">
          📸 Iniciar Captura de Pantalla
        </button>
        <button id="btn-preferences" style="background: #f1f5f9; color: #334155; border: 1px solid #cbd5e1; padding: 0.75rem 1.25rem; font-size: 1rem; font-weight: 600; border-radius: 8px; cursor: pointer; transition: background 0.2s;">
          ⚙️ Preferencias
        </button>
        <button id="btn-hide" style="background: #f8fafc; color: #64748b; border: 1px solid #e2e8f0; padding: 0.75rem 1.25rem; font-size: 1rem; font-weight: 600; border-radius: 8px; cursor: pointer; transition: background 0.2s;" title="Ocultar la ventana en segundo plano (puedes restaurarla desde el ícono de la bandeja)">
          🙈 Ocultar a la Bandeja
        </button>
      </div>
      <p style="font-size: 0.85rem; color: #64748b; margin-top: -0.5rem; margin-bottom: 1rem;">
        O presiona el atajo global: <kbd style="background: #e2e8f0; padding: 2px 6px; border-radius: 4px;">Command/Ctrl + Shift + X</kbd> (Cancela con <kbd style="background: #e2e8f0; padding: 2px 6px; border-radius: 4px;">Esc</kbd>)
      </p>

      <div id="selection-status" style="margin: 1rem 0; font-size: 0.9rem; color: #0f172a; min-height: 1.5rem;">
        Listo para capturar.
      </div>

      <div class="info-grid">
        <div class="info-item"><strong>Versión:</strong> ${appInfo.version}</div>
        <div class="info-item"><strong>Sistema Operativo:</strong> ${appInfo.os}</div>
        <div class="info-item"><strong>Monitor Principal:</strong> ${primary ? `${primary.logical_width}x${primary.logical_height} (${primary.scale_factor}x)` : "N/D"}</div>
        <div class="info-item"><strong>Privacidad:</strong> Verificada Local-First</div>
      </div>
    </div>
  `;

  const btnCapture = document.getElementById("btn-capture");
  if (btnCapture) {
    btnCapture.addEventListener("click", () => {
      void startSelection();
    });
  }

  const btnPreferences = document.getElementById("btn-preferences");
  if (btnPreferences) {
    btnPreferences.addEventListener("click", () => {
      void settingsModal?.open();
    });
  }

  const btnHide = document.getElementById("btn-hide");
  if (btnHide) {
    btnHide.addEventListener("click", async () => {
      try {
        const { getCurrentWindow } = await import("@tauri-apps/api/window");
        await getCurrentWindow().hide();
      } catch {}
    });
  }

  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const win = getCurrentWindow();
    await win.onCloseRequested(async (event) => {
      event.preventDefault();
      await win.hide();
    });

    void listen("open-preferences", async () => {
      try {
        await win.show();
        await win.setFocus();
      } catch {}
      void settingsModal?.open();
    });
    void listen("trigger-capture", async () => {
      void startSelection();
    });
  } catch {
    // Ignore in non-Tauri headless environments
  }
}

async function init() {
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const win = getCurrentWindow();
    if (win && win.label === "overlay") {
      await initOverlay();
      return;
    }
  } catch {}
  await renderApp();
}

void init();
