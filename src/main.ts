import "./App.css";
import { tauriBridge } from "./services/tauriBridge";

async function renderApp() {
  const root = document.getElementById("root");
  if (!root) return;

  const appInfo = await tauriBridge.getAppInfo();

  root.innerHTML = `
    <div class="container">
      <h1>Capturador de Pantalla</h1>
      <span class="badge-local-first">🛡️ 100% Local-First / Zero Network</span>
      <p>Herramienta corporativa de captura de pantalla y anotaciones de alta fidelidad.</p>
      
      <div class="info-grid">
        <div class="info-item"><strong>Versión:</strong> ${appInfo.version}</div>
        <div class="info-item"><strong>Entorno:</strong> ${appInfo.environment}</div>
        <div class="info-item"><strong>Sistema Operativo:</strong> ${appInfo.os}</div>
        <div class="info-item"><strong>Privacidad:</strong> Verificada Local-First</div>
      </div>
    </div>
  `;
}

renderApp();
