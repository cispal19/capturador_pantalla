export interface GifExportConfig {
  fps: number;
  maxDurationSeconds: number;
  repeatInfinite: boolean;
}

export interface GifExportResult {
  filePath: string;
  durationSeconds: number;
  fileSizeBytes: number;
  frameCount: number;
}
