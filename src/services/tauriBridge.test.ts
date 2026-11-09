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
});
