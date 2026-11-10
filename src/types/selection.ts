export type ResizeHandleType = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

export interface HandlePosition {
  handle: ResizeHandleType;
  x: number;
  y: number;
  cursor: string;
}

export interface SelectionRect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export type SelectionInteractionState =
  | { mode: "idle" }
  | { mode: "drawing"; startX: number; startY: number; currentX: number; currentY: number }
  | { mode: "selected"; box: SelectionRect }
  | {
      mode: "resizing";
      handle: ResizeHandleType;
      initialBox: SelectionRect;
      startX: number;
      startY: number;
      currentX: number;
      currentY: number;
    }
  | {
      mode: "moving";
      initialBox: SelectionRect;
      startX: number;
      startY: number;
      currentX: number;
      currentY: number;
    };

export interface SelectionOverlayConfig {
  dimOpacity: number;
  borderColor: string;
  borderWidth: number;
  handleSize: number;
  handleColor: string;
  handleBorderColor: string;
  badgeBgColor: string;
  badgeTextColor: string;
  backgroundImageUrl?: string;
  preloadedImage?: HTMLImageElement | null;
  displayId?: number;
  displays?: Array<{
    id: number;
    name: string;
    logical_width: number;
    logical_height: number;
    is_primary: boolean;
  }>;
  onSwitchDisplay?: (displayId: number) => void;
  restoreMainOnFinish?: boolean;
}

export const DEFAULT_OVERLAY_CONFIG: SelectionOverlayConfig = {
  dimOpacity: 0.4,
  borderColor: "#3b82f6",
  borderWidth: 2,
  handleSize: 8,
  handleColor: "#ffffff",
  handleBorderColor: "#1d4ed8",
  badgeBgColor: "rgba(15, 23, 42, 0.85)",
  badgeTextColor: "#ffffff",
};
