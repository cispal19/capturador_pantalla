use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GifExportConfig {
    pub fps: u32,
    pub max_duration_seconds: u32,
    pub repeat_infinite: bool,
}

impl Default for GifExportConfig {
    fn default() -> Self {
        Self {
            fps: 15,
            max_duration_seconds: 15,
            repeat_infinite: true,
        }
    }
}

impl GifExportConfig {
    pub fn new(fps: u32, max_duration_seconds: u32) -> Self {
        let safe_fps = fps.clamp(5, 24);
        let safe_duration = max_duration_seconds.clamp(1, 15);
        Self {
            fps: safe_fps,
            max_duration_seconds: safe_duration,
            repeat_infinite: true,
        }
    }

    pub fn frame_delay_ms(&self) -> u32 {
        1000 / self.fps.max(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GifExportResult {
    pub file_path: String,
    pub duration_seconds: u32,
    pub file_size_bytes: usize,
    pub frame_count: usize,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_gif_export_config_defaults() {
        let cfg = GifExportConfig::default();
        assert_eq!(cfg.fps, 15);
        assert_eq!(cfg.max_duration_seconds, 15);
        assert!(cfg.repeat_infinite);
        assert_eq!(cfg.frame_delay_ms(), 66); // 1000 / 15 = 66ms
    }

    #[test]
    fn test_gif_export_config_clamping() {
        let under = GifExportConfig::new(2, 0);
        assert_eq!(under.fps, 5);
        assert_eq!(under.max_duration_seconds, 1);

        let over = GifExportConfig::new(60, 60);
        assert_eq!(over.fps, 24);
        assert_eq!(over.max_duration_seconds, 15);
    }

    #[test]
    fn test_gif_export_result_serialization() {
        let res = GifExportResult {
            file_path: "/tmp/Animacion_test.gif".to_string(),
            duration_seconds: 3,
            file_size_bytes: 52400,
            frame_count: 45,
        };

        let json = serde_json::to_string(&res).unwrap();
        let parsed: GifExportResult = serde_json::from_str(&json).unwrap();
        assert_eq!(res, parsed);
    }
}
