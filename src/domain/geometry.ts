import type { HandlePosition, ResizeHandleType, SelectionRect } from "../types/selection";

export function normalizeSelection(x1: number, y1: number, x2: number, y2: number): SelectionRect {
  const x = Math.min(x1, x2);
  const y = Math.min(y1, y2);
  const width = Math.abs(x2 - x1);
  const height = Math.abs(y2 - y1);
  return { x, y, width, height };
}

export function clampRect(rect: SelectionRect, maxWidth: number, maxHeight: number): SelectionRect {
  const clampedX = Math.max(0, Math.min(rect.x, maxWidth));
  const clampedY = Math.max(0, Math.min(rect.y, maxHeight));

  const availableW = Math.max(0, maxWidth - clampedX);
  const availableH = Math.max(0, maxHeight - clampedY);

  const clampedW = Math.min(rect.width, availableW);
  const clampedH = Math.min(rect.height, availableH);

  return {
    x: clampedX,
    y: clampedY,
    width: clampedW,
    height: clampedH,
  };
}

export function getHandleAnchors(rect: SelectionRect): HandlePosition[] {
  const { x, y, width, height } = rect;
  const halfW = width / 2;
  const halfH = height / 2;

  return [
    { handle: "nw", x, y, cursor: "nwse-resize" },
    { handle: "n", x: x + halfW, y, cursor: "ns-resize" },
    { handle: "ne", x: x + width, y, cursor: "nesw-resize" },
    { handle: "e", x: x + width, y: y + halfH, cursor: "ew-resize" },
    { handle: "se", x: x + width, y: y + height, cursor: "nwse-resize" },
    { handle: "s", x: x + halfW, y: y + height, cursor: "ns-resize" },
    { handle: "sw", x, y: y + height, cursor: "nesw-resize" },
    { handle: "w", x, y: y + halfH, cursor: "ew-resize" },
  ];
}

export function hitTestHandle(
  rect: SelectionRect,
  px: number,
  py: number,
  tolerance = 8
): ResizeHandleType | null {
  const anchors = getHandleAnchors(rect);
  for (const anchor of anchors) {
    if (Math.abs(px - anchor.x) <= tolerance && Math.abs(py - anchor.y) <= tolerance) {
      return anchor.handle;
    }
  }
  return null;
}

export function pointInRect(rect: SelectionRect, px: number, py: number): boolean {
  return px >= rect.x && px <= rect.x + rect.width && py >= rect.y && py <= rect.y + rect.height;
}

export function resizeByHandle(
  rect: SelectionRect,
  handle: ResizeHandleType,
  dx: number,
  dy: number,
  minSize = 10
): SelectionRect {
  let { x, y, width, height } = rect;

  switch (handle) {
    case "nw": {
      const newW = Math.max(minSize, width - dx);
      x += width - newW;
      width = newW;

      const newH = Math.max(minSize, height - dy);
      y += height - newH;
      height = newH;
      break;
    }
    case "n": {
      const newH = Math.max(minSize, height - dy);
      y += height - newH;
      height = newH;
      break;
    }
    case "ne": {
      width = Math.max(minSize, width + dx);
      const newH = Math.max(minSize, height - dy);
      y += height - newH;
      height = newH;
      break;
    }
    case "e": {
      width = Math.max(minSize, width + dx);
      break;
    }
    case "se": {
      width = Math.max(minSize, width + dx);
      height = Math.max(minSize, height + dy);
      break;
    }
    case "s": {
      height = Math.max(minSize, height + dy);
      break;
    }
    case "sw": {
      const newW = Math.max(minSize, width - dx);
      x += width - newW;
      width = newW;
      height = Math.max(minSize, height + dy);
      break;
    }
    case "w": {
      const newW = Math.max(minSize, width - dx);
      x += width - newW;
      width = newW;
      break;
    }
  }

  return { x, y, width, height };
}

export function translateRect(
  rect: SelectionRect,
  dx: number,
  dy: number,
  maxWidth: number,
  maxHeight: number
): SelectionRect {
  const maxX = Math.max(0, maxWidth - rect.width);
  const maxY = Math.max(0, maxHeight - rect.height);

  const x = Math.max(0, Math.min(rect.x + dx, maxX));
  const y = Math.max(0, Math.min(rect.y + dy, maxY));

  return {
    x,
    y,
    width: rect.width,
    height: rect.height,
  };
}
