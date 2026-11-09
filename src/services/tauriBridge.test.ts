import { describe, it, expect } from "vitest";
import { tauriBridge } from "./tauriBridge";

describe("TauriBridgeService", () => {
  it("retrieves app info with local-first verified flag", async () => {
    const info = await tauriBridge.getAppInfo();
    expect(info.name).toBe("capturador-pantalla");
    expect(info.version).toBe("1.0.0");
    expect(info.local_first_verified).toBe(true);
  });

  it("normalizes bounding box from inverted drag coordinates", async () => {
    const p1 = { x: 500, y: 400 };
    const p2 = { x: 100, y: 50 };
    const scaleFactor = 2.0;

    const bbox = await tauriBridge.normalizeBoundingBox(p1, p2, scaleFactor);

    expect(bbox.origin.x).toBe(100);
    expect(bbox.origin.y).toBe(50);
    expect(bbox.size.width).toBe(400);
    expect(bbox.size.height).toBe(350);
    expect(bbox.physical_rect.x).toBe(200);
    expect(bbox.physical_rect.y).toBe(100);
    expect(bbox.physical_rect.width).toBe(800);
    expect(bbox.physical_rect.height).toBe(700);
  });

  it("retrieves default idle session state", async () => {
    const session = await tauriBridge.getSessionState();
    expect(session.state).toBe("Idle");
  });

  it("retrieves connected monitors metadata", async () => {
    const monitors = await tauriBridge.getConnectedMonitors();
    expect(monitors.length).toBeGreaterThan(0);
    expect(monitors[0].id).toBe(0);
    expect(monitors[0].scale_factor).toBe(2.0);
    expect(monitors[0].is_primary).toBe(true);
  });

  it("triggers screen capture session successfully", async () => {
    const result = await tauriBridge.triggerScreenCapture(0);
    expect(result.success).toBe(true);
    expect(result.display_id).toBe(0);
    expect(result.session_state).toBe("Selecting");
  });

  it("registers global capture shortcut accelerator", async () => {
    const res = await tauriBridge.registerCaptureShortcut("CommandOrControl+Shift+X");
    expect(res.registered).toBe(true);
    expect(res.accelerator).toBe("CommandOrControl+Shift+X");
  });

  it("cancels capture session returning Idle state", async () => {
    const res = await tauriBridge.cancelCaptureSession();
    expect(res.state).toBe("Idle");
  });

  it("commits selection region successfully", async () => {
    const bbox = await tauriBridge.commitSelectionRegion(100, 150, 600, 400, 2.0);
    expect(bbox.origin.x).toBe(100);
    expect(bbox.origin.y).toBe(150);
    expect(bbox.size.width).toBe(600);
    expect(bbox.size.height).toBe(400);
    expect(bbox.physical_rect.width).toBe(1200);
    expect(bbox.physical_rect.height).toBe(800);
  });

  it("applies obfuscation filter", async () => {
    const success = await tauriBridge.applyObfuscation(50, 50, 200, 100, 12);
    expect(success).toBe(true);
  });

  it("synchronizes vector annotations to backend", async () => {
    const count = await tauriBridge.syncAnnotations([{ type: "rectangle", id: "1" }]);
    expect(count).toBe(1);
  });

  it("copies captured screenshot to OS clipboard", async () => {
    const success = await tauriBridge.copyCaptureToClipboard(10, 10, new Uint8ClampedArray(400));
    expect(success).toBe(true);
  });

  it("saves captured screenshot to local file", async () => {
    const res = await tauriBridge.saveCaptureToFile(10, 10, new Uint8ClampedArray(400), "png");
    expect(res.format).toBe("png");
    expect(res.filePath).toBeDefined();
    expect(res.fileSizeBytes).toBe(400);
  });

  it("fetches, updates, and resets user preferences", async () => {
    const prefs = await tauriBridge.getPreferences();
    expect(prefs.capture_shortcut).toBeDefined();
    expect(prefs.default_image_format).toBe("png");

    const updated = await tauriBridge.updatePreferences({
      ...prefs,
      default_image_format: "jpeg",
      autostart_enabled: true,
    });
    expect(updated.default_image_format).toBe("jpeg");
    expect(updated.autostart_enabled).toBe(true);

    const reset = await tauriBridge.resetPreferences();
    expect(reset.default_image_format).toBe("png");
    expect(reset.autostart_enabled).toBe(false);
  });

  it("handles screen recording lifecycle via tauriBridge (Feature 007)", async () => {
    const region = await tauriBridge.startScreenRecording(100, 100, 401, 301, 30, 300);
    expect(region.width).toBe(400); // Aligned to even
    expect(region.height).toBe(300);

    const paused = await tauriBridge.pauseScreenRecording();
    expect(paused).toBe(true);

    const resumed = await tauriBridge.resumeScreenRecording();
    expect(resumed).toBe(true);

    const meta = await tauriBridge.stopScreenRecording(new Uint8Array([1, 2, 3, 4]), 5, "webm");
    expect(meta.durationSeconds).toBe(5);
    expect(meta.format).toBe("webm");

    const cancelled = await tauriBridge.cancelScreenRecording();
    expect(cancelled).toBe(true);

    const status = await tauriBridge.getRecordingStatus();
    expect(status.state).toBeDefined();
  });

  it("exports frames to animated GIF via tauriBridge (Feature 008)", async () => {
    const frame = new Uint8ClampedArray(10 * 10 * 4);
    frame.fill(128);
    const result = await tauriBridge.exportFramesToGif(10, 10, [frame, frame], 15);
    expect(result.filePath).toContain(".gif");
    expect(result.frameCount).toBe(2);
    expect(result.durationSeconds).toBeCloseTo(2 / 15, 2);
    expect(result.fileSizeBytes).toBeGreaterThan(0);
  });
});
