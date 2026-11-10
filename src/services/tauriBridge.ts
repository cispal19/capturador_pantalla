import { invoke } from "@tauri-apps/api/core";
import { type AppPreferences, DEFAULT_PREFERENCES } from "../types/preferences";
import type { RecordingMetadata, RecordingRegion, RecordingState } from "../types/recording";
import type { GifExportResult } from "../types/gif";
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

  async startCapture(displayId?: number): Promise<CaptureTriggerResult> {
    try {
      return await invoke<CaptureTriggerResult>("start_capture_session_flow", {
        displayId,
      });
    } catch {
      return this.triggerScreenCapture(displayId);
    }
  }

  async hideCaptureOverlay(): Promise<void> {
    try {
      await invoke("hide_capture_overlay");
    } catch {
      // Ignored outside Tauri
    }
  }

  async finishCapture(restoreMain = false): Promise<void> {
    try {
      await invoke("finish_capture_session_flow", { restoreMain });
    } catch {
      await this.hideCaptureOverlay();
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

  async applyObfuscation(
    rectX: number,
    rectY: number,
    width: number,
    height: number,
    blockSize = 12
  ): Promise<boolean> {
    try {
      return await invoke<boolean>("apply_obfuscation", {
        rectX,
        rectY,
        width,
        height,
        blockSize,
      });
    } catch {
      return true;
    }
  }

  async syncAnnotations(annotations: unknown[]): Promise<number> {
    try {
      return await invoke<number>("sync_annotations", { annotations });
    } catch {
      return annotations.length;
    }
  }

  async copyCaptureToClipboard(
    width: number,
    height: number,
    rgbaData: number[] | Uint8ClampedArray
  ): Promise<boolean> {
    try {
      const data = Array.isArray(rgbaData) ? rgbaData : Array.from(rgbaData);
      return await invoke<boolean>("copy_capture_to_clipboard", {
        width: Math.round(width),
        height: Math.round(height),
        rgbaData: data,
      });
    } catch {
      return true;
    }
  }

  async saveCaptureToFile(
    width: number,
    height: number,
    rgbaData: number[] | Uint8ClampedArray,
    format = "png",
    targetDirectory?: string
  ): Promise<{ filePath: string; fileSizeBytes: number; format: string }> {
    try {
      const data = Array.isArray(rgbaData) ? rgbaData : Array.from(rgbaData);
      return await invoke<{ filePath: string; fileSizeBytes: number; format: string }>(
        "save_capture_to_file",
        {
          width: Math.round(width),
          height: Math.round(height),
          rgbaData: data,
          format,
          targetDirectory,
        }
      );
    } catch {
      return {
        filePath: `/Users/mock/Pictures/Captura_mock.${format}`,
        fileSizeBytes: rgbaData.length,
        format,
      };
    }
  }

  async getPreferences(): Promise<AppPreferences> {
    try {
      return await invoke<AppPreferences>("get_preferences");
    } catch {
      return { ...DEFAULT_PREFERENCES };
    }
  }

  async updatePreferences(preferences: AppPreferences): Promise<AppPreferences> {
    try {
      return await invoke<AppPreferences>("update_preferences", { preferences });
    } catch {
      return { ...preferences };
    }
  }

  async resetPreferences(): Promise<AppPreferences> {
    try {
      return await invoke<AppPreferences>("reset_preferences");
    } catch {
      return { ...DEFAULT_PREFERENCES };
    }
  }

  async startScreenRecording(
    x: number,
    y: number,
    width: number,
    height: number,
    fps = 30,
    timeLimitSeconds = 600
  ): Promise<RecordingRegion> {
    try {
      return await invoke<RecordingRegion>("start_screen_recording", {
        x: Math.round(x),
        y: Math.round(y),
        width: Math.round(width),
        height: Math.round(height),
        fps,
        timeLimitSeconds,
      });
    } catch {
      return {
        x: Math.round(x),
        y: Math.round(y),
        width: Math.round(width) & ~1,
        height: Math.round(height) & ~1,
        fps,
        timeLimitSeconds,
      };
    }
  }

  async pauseScreenRecording(): Promise<boolean> {
    try {
      return await invoke<boolean>("pause_screen_recording");
    } catch {
      return true;
    }
  }

  async resumeScreenRecording(): Promise<boolean> {
    try {
      return await invoke<boolean>("resume_screen_recording");
    } catch {
      return true;
    }
  }

  async stopScreenRecording(
    videoData: Uint8Array | number[],
    durationSeconds: number,
    format = "webm"
  ): Promise<RecordingMetadata> {
    try {
      const data = Array.isArray(videoData) ? videoData : Array.from(videoData);
      return await invoke<RecordingMetadata>("stop_screen_recording", {
        videoData: data,
        durationSeconds: Math.round(durationSeconds),
        format,
      });
    } catch {
      return {
        filePath: `/Users/mock/Pictures/Grabacion_mock.${format}`,
        durationSeconds: Math.round(durationSeconds),
        fileSizeBytes: videoData.length,
        format,
        fps: 30,
      };
    }
  }

  async cancelScreenRecording(): Promise<boolean> {
    try {
      return await invoke<boolean>("cancel_screen_recording");
    } catch {
      return true;
    }
  }

  async getRecordingStatus(): Promise<{
    state: RecordingState;
    elapsed_seconds: number;
    region: RecordingRegion | null;
  }> {
    try {
      return await invoke<{
        state: RecordingState;
        elapsed_seconds: number;
        region: RecordingRegion | null;
      }>("get_recording_status");
    } catch {
      return {
        state: "Idle",
        elapsed_seconds: 0,
        region: null,
      };
    }
  }

  async exportFramesToGif(
    width: number,
    height: number,
    framesRgba: number[][] | Uint8ClampedArray[] | Uint8Array[],
    fps = 15,
    targetDirectory?: string
  ): Promise<GifExportResult> {
    try {
      const serializableFrames = framesRgba.map((f) => (Array.isArray(f) ? f : Array.from(f)));
      return await invoke<GifExportResult>("export_frames_to_gif", {
        width: Math.round(width),
        height: Math.round(height),
        framesRgba: serializableFrames,
        fps,
        targetDirectory,
      });
    } catch {
      return {
        filePath: `/Users/mock/Pictures/Animacion_mock.gif`,
        durationSeconds: framesRgba.length / fps,
        fileSizeBytes: framesRgba.length * width * height * 0.1,
        frameCount: framesRgba.length,
      };
    }
  }
}

export const tauriBridge = new TauriBridgeService();
