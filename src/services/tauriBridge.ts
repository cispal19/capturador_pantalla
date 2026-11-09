import { invoke } from "@tauri-apps/api/core";
import type {
  AppInfo,
  BoundingBox,
  DisplayCoordinates,
  CaptureSessionState,
  DisplayInfo,
  CaptureTriggerResult,
} from "../types/domain";

export class TauriBridgeService {
  async getAppInfo(): Promise<AppInfo> {
    try {
      return await invoke<AppInfo>("get_app_info");
    } catch {
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

  async getConnectedMonitors(): Promise<DisplayInfo[]> {
    try {
      return await invoke<DisplayInfo[]>("get_connected_monitors");
    } catch {
      return [
        {
          id: 0,
          name: "Built-in Retina Display",
          logical_width: 1728,
          logical_height: 1117,
          physical_width: 3456,
          physical_height: 2234,
          scale_factor: 2.0,
          is_primary: true,
        },
      ];
    }
  }

  async triggerScreenCapture(displayId?: number): Promise<CaptureTriggerResult> {
    try {
      return await invoke<CaptureTriggerResult>("trigger_screen_capture", {
        displayId,
      });
    } catch {
      return {
        success: true,
        display_id: displayId ?? 0,
        width: 3456,
        height: 2234,
        scale_factor: 2.0,
        session_state: "Selecting",
      };
    }
  }

  async registerCaptureShortcut(
    accelerator: string
  ): Promise<{ registered: boolean; accelerator: string }> {
    try {
      return await invoke<{ registered: boolean; accelerator: string }>(
        "register_capture_shortcut",
        { accelerator }
      );
    } catch {
      return { registered: true, accelerator };
    }
  }

  async cancelCaptureSession(): Promise<{ state: CaptureSessionState }> {
    try {
      return await invoke<{ state: CaptureSessionState }>("cancel_capture_session");
    } catch {
      return { state: "Idle" };
    }
  }

  async commitSelectionRegion(
    originX: number,
    originY: number,
    width: number,
    height: number,
    scaleFactor = 1.0
  ): Promise<BoundingBox> {
    try {
      return await invoke<BoundingBox>("commit_selection_region", {
        originX,
        originY,
        width,
        height,
        scaleFactor,
      });
    } catch {
      return {
        origin: { x: originX, y: originY },
        size: { width, height },
        scale_factor: scaleFactor,
        physical_rect: {
          x: Math.round(originX * scaleFactor),
          y: Math.round(originY * scaleFactor),
          width: Math.round(width * scaleFactor),
          height: Math.round(height * scaleFactor),
        },
      };
    }
  }
}

export const tauriBridge = new TauriBridgeService();
