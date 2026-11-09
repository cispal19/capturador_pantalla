use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RecordingState {
    #[default]
    Idle,
    Recording,
    Paused,
    Finalizing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordingRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub time_limit_seconds: u32,
}

impl RecordingRegion {
    pub fn new(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        fps: u32,
        time_limit_seconds: u32,
    ) -> Result<Self, String> {
        if width < 50 || height < 50 {
            return Err("Las dimensiones mínimas de grabación son 50x50 píxeles".to_string());
        }

        // Align dimensions to even values for video container / codec compatibility
        let even_width = width & !1;
        let even_height = height & !1;

        let safe_fps = if fps == 0 { 30 } else { fps.min(60) };
        let safe_limit = if time_limit_seconds == 0 {
            600
        } else {
            time_limit_seconds.min(3600)
        };

        Ok(Self {
            x,
            y,
            width: even_width,
            height: even_height,
            fps: safe_fps,
            time_limit_seconds: safe_limit,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub file_path: String,
    pub duration_seconds: u32,
    pub file_size_bytes: usize,
    pub format: String,
    pub fps: u32,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_recording_region_valid_and_even_alignment() {
        // Odd numbers 501x399 should be aligned down to 500x398
        let region = RecordingRegion::new(100, 150, 501, 399, 30, 300).unwrap();
        assert_eq!(region.x, 100);
        assert_eq!(region.y, 150);
        assert_eq!(region.width, 500);
        assert_eq!(region.height, 398);
        assert_eq!(region.fps, 30);
        assert_eq!(region.time_limit_seconds, 300);
    }

    #[test]
    fn test_recording_region_rejects_sub_minimum() {
        assert!(RecordingRegion::new(0, 0, 49, 100, 30, 300).is_err());
        assert!(RecordingRegion::new(0, 0, 100, 48, 30, 300).is_err());
    }

    #[test]
    fn test_recording_region_safe_fps_and_time_limits() {
        let r1 = RecordingRegion::new(0, 0, 100, 100, 0, 0).unwrap();
        assert_eq!(r1.fps, 30);
        assert_eq!(r1.time_limit_seconds, 600);

        let r2 = RecordingRegion::new(0, 0, 100, 100, 120, 5000).unwrap();
        assert_eq!(r2.fps, 60);
        assert_eq!(r2.time_limit_seconds, 3600);
    }

    #[test]
    fn test_recording_state_variants_and_serialization() {
        let states = vec![
            RecordingState::Idle,
            RecordingState::Recording,
            RecordingState::Paused,
            RecordingState::Finalizing,
        ];

        for s in states {
            let json = serde_json::to_string(&s).unwrap();
            let parsed: RecordingState = serde_json::from_str(&json).unwrap();
            assert_eq!(s, parsed);
        }
    }
}
