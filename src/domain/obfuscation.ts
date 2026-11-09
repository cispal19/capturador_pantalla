/**
 * Destructive block-averaging pixelation algorithm for Canvas ImageData buffers.
 * Irreversibly destroys high-frequency features (text, tokens, PII).
 */
export function applyBlockPixelation(
  data: Uint8ClampedArray,
  bufWidth: number,
  bufHeight: number,
  rectX: number,
  rectY: number,
  rectW: number,
  rectH: number,
  blockSize = 12
): void {
  if (blockSize <= 0 || bufWidth <= 0 || bufHeight <= 0 || rectW <= 0 || rectH <= 0) {
    return;
  }

  const startX = Math.max(0, Math.min(Math.round(rectX), bufWidth));
  const startY = Math.max(0, Math.min(Math.round(rectY), bufHeight));
  const maxX = Math.max(0, Math.min(Math.round(rectX + rectW), bufWidth));
  const maxY = Math.max(0, Math.min(Math.round(rectY + rectH), bufHeight));

  for (let by = startY; by < maxY; by += blockSize) {
    const byEnd = Math.min(by + blockSize, maxY);

    for (let bx = startX; bx < maxX; bx += blockSize) {
      const bxEnd = Math.min(bx + blockSize, maxX);

      let sumR = 0;
      let sumG = 0;
      let sumB = 0;
      let count = 0;

      for (let y = by; y < byEnd; y++) {
        for (let x = bx; x < bxEnd; x++) {
          const offset = (y * bufWidth + x) * 4;
          if (offset + 3 < data.length) {
            sumR += data[offset];
            sumG += data[offset + 1];
            sumB += data[offset + 2];
            count++;
          }
        }
      }

      if (count > 0) {
        const avgR = Math.floor(sumR / count);
        const avgG = Math.floor(sumG / count);
        const avgB = Math.floor(sumB / count);

        for (let y = by; y < byEnd; y++) {
          for (let x = bx; x < bxEnd; x++) {
            const offset = (y * bufWidth + x) * 4;
            if (offset + 3 < data.length) {
              data[offset] = avgR;
              data[offset + 1] = avgG;
              data[offset + 2] = avgB;
              // Preserve alpha channel
            }
          }
        }
      }
    }
  }
}
