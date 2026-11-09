import { describe, it, expect } from "vitest";
import {
  normalizeSelection,
  clampRect,
  getHandleAnchors,
  hitTestHandle,
  resizeByHandle,
  translateRect,
  pointInRect,
} from "./geometry";
import type { SelectionRect } from "../types/selection";

describe("Geometry Domain Functions (Frontend)", () => {
  describe("normalizeSelection", () => {
    it("normalizes standard top-left to bottom-right drag", () => {
      const rect = normalizeSelection(100, 100, 400, 300);
      expect(rect).toEqual({ x: 100, y: 100, width: 300, height: 200 });
    });

    it("normalizes inverted bottom-right to top-left drag", () => {
      const rect = normalizeSelection(400, 300, 100, 100);
      expect(rect).toEqual({ x: 100, y: 100, width: 300, height: 200 });
    });

    it("handles zero drag area", () => {
      const rect = normalizeSelection(150, 150, 150, 150);
      expect(rect).toEqual({ x: 150, y: 150, width: 0, height: 0 });
    });
  });

  describe("clampRect", () => {
    it("clamps rect strictly inside display boundary", () => {
      const rect: SelectionRect = { x: -20, y: -10, width: 2000, height: 1200 };
      const clamped = clampRect(rect, 1920, 1080);
      expect(clamped).toEqual({ x: 0, y: 0, width: 1920, height: 1080 });
    });
  });

  describe("getHandleAnchors", () => {
    it("generates 8 canonical handle positions with cursor mappings", () => {
      const rect: SelectionRect = { x: 100, y: 200, width: 400, height: 300 };
      const anchors = getHandleAnchors(rect);

      expect(anchors).toHaveLength(8);

      const nw = anchors.find((a) => a.handle === "nw");
      expect(nw).toEqual({ handle: "nw", x: 100, y: 200, cursor: "nwse-resize" });

      const n = anchors.find((a) => a.handle === "n");
      expect(n).toEqual({ handle: "n", x: 300, y: 200, cursor: "ns-resize" });

      const se = anchors.find((a) => a.handle === "se");
      expect(se).toEqual({ handle: "se", x: 500, y: 500, cursor: "nwse-resize" });
    });
  });

  describe("hitTestHandle", () => {
    it("identifies handle within tolerance radius", () => {
      const rect: SelectionRect = { x: 100, y: 200, width: 400, height: 300 };
      // SE is at (500, 500)
      const hit = hitTestHandle(rect, 502, 498, 8);
      expect(hit).toBe("se");

      // Miss
      expect(hitTestHandle(rect, 200, 200, 8)).toBeNull();
    });
  });

  describe("pointInRect", () => {
    it("detects point interior", () => {
      const rect: SelectionRect = { x: 100, y: 100, width: 200, height: 200 };
      expect(pointInRect(rect, 150, 150)).toBe(true);
      expect(pointInRect(rect, 50, 50)).toBe(false);
    });
  });

  describe("resizeByHandle", () => {
    it("resizes from SE handle", () => {
      const initial: SelectionRect = { x: 100, y: 100, width: 200, height: 200 };
      const resized = resizeByHandle(initial, "se", 50, 30, 10);
      expect(resized).toEqual({ x: 100, y: 100, width: 250, height: 230 });
    });

    it("resizes from NW handle and clamps to min size", () => {
      const initial: SelectionRect = { x: 100, y: 100, width: 200, height: 200 };
      const resized = resizeByHandle(initial, "nw", 300, 300, 10);
      expect(resized.width).toBe(10);
      expect(resized.height).toBe(10);
    });
  });

  describe("translateRect", () => {
    it("translates rect maintaining dimensions within display limits", () => {
      const initial: SelectionRect = { x: 100, y: 100, width: 200, height: 100 };
      const translated = translateRect(initial, 50, 30, 1920, 1080);
      expect(translated).toEqual({ x: 150, y: 130, width: 200, height: 100 });

      // Translate out of bounds clamped
      const clampedMove = translateRect(initial, 3000, 3000, 1920, 1080);
      expect(clampedMove).toEqual({ x: 1720, y: 980, width: 200, height: 100 });
    });
  });
});
