export interface DisplayCoordinates {
  x: number;
  y: number;
}

export interface Dimensions {
  width: number;
  height: number;
}

export interface BoundingBox {
  origin: DisplayCoordinates;
  size: Dimensions;
  scale_factor: number;
  physical_rect: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
}

export type CaptureSessionState = "Idle" | "Selecting" | "Completed" | "Cancelled";

export interface AppInfo {
  name: string;
  version: string;
  environment: "development" | "production";
  os: "macos" | "windows" | "linux";
  local_first_verified: boolean;
}
