import { invoke } from "@tauri-apps/api/core";
import type {
  AppInfo,
  BoundingBox,
  DisplayCoordinates,
  CaptureSessionState,
} from "../types/domain";

export class TauriBridgeService {
  async getAppInfo(): Promise<AppInfo> {
    try {
      return await invoke<AppInfo>("get_app_info");
    } catch {
      // Fallback for browser / test mock environment
      return {
        name: "capturador-pantalla",
        version: "1.0.0",
        environment: "development",
        os: "macos",
        local_first_verified: true,
      };
    }
  }

  async normalizeBoundingBox(
    p1: DisplayCoordinates,
    p2: DisplayCoordinates,
    scaleFactor: number
  ): Promise<BoundingBox> {
    try {
      return await invoke<BoundingBox>("normalize_bounding_box", {
        p1,
        p2,
        scaleFactor,
      });
    } catch {
      const minX = Math.min(p1.x, p2.x);
      const minY = Math.min(p1.y, p2.y);
      const width = Math.abs(p2.x - p1.x);
      const height = Math.abs(p2.y - p1.y);
      return {
        origin: { x: minX, y: minY },
        size: { width, height },
        scale_factor: scaleFactor,
        physical_rect: {
          x: Math.round(minX * scaleFactor),
          y: Math.round(minY * scaleFactor),
          width: Math.round(width * scaleFactor),
          height: Math.round(height * scaleFactor),
        },
      };
    }
  }

  async getSessionState(): Promise<{ state: CaptureSessionState }> {
    try {
      return await invoke<{ state: CaptureSessionState }>("get_session_state");
    } catch {
      return { state: "Idle" };
    }
  }
}

export const tauriBridge = new TauriBridgeService();
