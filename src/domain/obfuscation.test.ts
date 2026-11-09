import { describe, it, expect } from "vitest";
import { applyBlockPixelation } from "./obfuscation";

describe("applyBlockPixelation (Frontend)", () => {
  it("averages pixel blocks destroying high-frequency variations", () => {
    // Create an 8x8 imageData buffer with 4 bytes per pixel (RGBA)
    const width = 8;
    const height = 8;
    const data = new Uint8ClampedArray(width * height * 4);

    // Alternating 255 and 0
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const offset = (y * width + x) * 4;
        const val = (x + y) % 2 === 0 ? 255 : 0;
        data[offset] = val;
        data[offset + 1] = val;
        data[offset + 2] = val;
        data[offset + 3] = 255;
      }
    }

    applyBlockPixelation(data, width, height, 0, 0, width, height, 4);

    // Expect all pixels in the first 4x4 block to have flattened mean of 127
    for (let y = 0; y < 4; y++) {
      for (let x = 0; x < 4; x++) {
        const offset = (y * width + x) * 4;
        expect(data[offset]).toBe(127);
        expect(data[offset + 1]).toBe(127);
        expect(data[offset + 2]).toBe(127);
        expect(data[offset + 3]).toBe(255);
      }
    }
  });

  it("safely clamps when obfuscation rectangle extends beyond boundaries", () => {
    const width = 10;
    const height = 10;
    const data = new Uint8ClampedArray(width * height * 4);

    expect(() => {
      applyBlockPixelation(data, width, height, 5, 5, 50, 50, 4);
    }).not.toThrow();
  });
});
