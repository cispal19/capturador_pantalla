export type RecordingState = "Idle" | "Recording" | "Paused" | "Finalizing";

export interface RecordingRegion {
  x: number;
  y: number;
  width: number;
  height: number;
  fps: number;
  timeLimitSeconds: number;
}

export interface RecordingMetadata {
  filePath: string;
  durationSeconds: number;
  fileSizeBytes: number;
  format: string;
  fps: number;
}
