use crate::domain::naming::FileNameGenerator;
use crate::domain::recording::{RecordingMetadata, RecordingRegion, RecordingState};
use crate::ports::{IScreenRecorderDriver, RecorderDriverError};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[derive(Clone, Default)]
pub struct MockScreenRecorderDriver {
    state: Arc<Mutex<RecordingState>>,
    active_region: Arc<Mutex<Option<RecordingRegion>>>,
    target_directory: Option<PathBuf>,
}

impl MockScreenRecorderDriver {
    pub fn new(target_directory: Option<PathBuf>) -> Self {
        Self {
            state: Arc::new(Mutex::new(RecordingState::Idle)),
            active_region: Arc::new(Mutex::new(None)),
            target_directory,
        }
    }

    pub fn get_active_region(&self) -> Option<RecordingRegion> {
        self.active_region.lock().unwrap().clone()
    }
}

impl IScreenRecorderDriver for MockScreenRecorderDriver {
    fn start(&self, region: &RecordingRegion) -> Result<(), RecorderDriverError> {
        let mut st = self.state.lock().unwrap();
        if *st != RecordingState::Idle {
            return Err(RecorderDriverError::AlreadyRecording);
        }
        *st = RecordingState::Recording;
        let mut reg = self.active_region.lock().unwrap();
        *reg = Some(region.clone());
        Ok(())
    }

    fn pause(&self) -> Result<(), RecorderDriverError> {
        let mut st = self.state.lock().unwrap();
        if *st != RecordingState::Recording {
            return Err(RecorderDriverError::InvalidState(*st));
        }
        *st = RecordingState::Paused;
        Ok(())
    }

    fn resume(&self) -> Result<(), RecorderDriverError> {
        let mut st = self.state.lock().unwrap();
        if *st != RecordingState::Paused {
            return Err(RecorderDriverError::InvalidState(*st));
        }
        *st = RecordingState::Recording;
        Ok(())
    }

    fn stop(
        &self,
        video_bytes: &[u8],
        duration_seconds: u32,
        format: &str,
    ) -> Result<RecordingMetadata, RecorderDriverError> {
        let mut st = self.state.lock().unwrap();
        if *st == RecordingState::Idle {
            return Err(RecorderDriverError::NoActiveSession);
        }
        *st = RecordingState::Finalizing;

        let dir = self.target_directory.clone().unwrap_or_else(env::temp_dir);

        let now_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let clean_ext = if format.is_empty() { "webm" } else { format };
        let file_name = FileNameGenerator::generate_recording_name(clean_ext, now_secs);
        let file_path = dir.join(file_name);

        let _ = fs::create_dir_all(&dir);
        fs::write(&file_path, video_bytes)
            .map_err(|e| RecorderDriverError::EncodingError(e.to_string()))?;

        let region = self.active_region.lock().unwrap().clone();
        let fps = region.map(|r| r.fps).unwrap_or(30);

        *st = RecordingState::Idle;
        *self.active_region.lock().unwrap() = None;

        Ok(RecordingMetadata {
            file_path: file_path.to_string_lossy().to_string(),
            duration_seconds,
            file_size_bytes: video_bytes.len(),
            format: clean_ext.to_string(),
            fps,
        })
    }

    fn cancel(&self) -> Result<(), RecorderDriverError> {
        let mut st = self.state.lock().unwrap();
        *st = RecordingState::Idle;
        *self.active_region.lock().unwrap() = None;
        Ok(())
    }

    fn state(&self) -> RecordingState {
        *self.state.lock().unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_mock_recorder_full_lifecycle() {
        let temp_dir = env::temp_dir().join("test_mock_recorder");
        let _ = fs::remove_dir_all(&temp_dir);

        let driver = MockScreenRecorderDriver::new(Some(temp_dir.clone()));
        assert_eq!(driver.state(), RecordingState::Idle);

        let region = RecordingRegion::new(100, 100, 400, 300, 30, 60).unwrap();
        driver.start(&region).unwrap();
        assert_eq!(driver.state(), RecordingState::Recording);
        assert_eq!(driver.get_active_region(), Some(region));

        // Duplicate start rejected
        assert_eq!(
            driver.start(&RecordingRegion::new(0, 0, 100, 100, 30, 60).unwrap()),
            Err(RecorderDriverError::AlreadyRecording)
        );

        // Pause
        driver.pause().unwrap();
        assert_eq!(driver.state(), RecordingState::Paused);

        // Resume
        driver.resume().unwrap();
        assert_eq!(driver.state(), RecordingState::Recording);

        // Stop
        let fake_bytes = vec![0x1A, 0x45, 0xDF, 0xA3]; // WebM EBML header mock
        let meta = driver.stop(&fake_bytes, 5, "webm").unwrap();
        assert_eq!(driver.state(), RecordingState::Idle);
        assert_eq!(meta.duration_seconds, 5);
        assert_eq!(meta.file_size_bytes, 4);
        assert!(std::path::Path::new(&meta.file_path).exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_mock_recorder_cancel() {
        let driver = MockScreenRecorderDriver::new(None);
        let region = RecordingRegion::new(0, 0, 200, 200, 30, 60).unwrap();
        driver.start(&region).unwrap();
        assert_eq!(driver.state(), RecordingState::Recording);

        driver.cancel().unwrap();
        assert_eq!(driver.state(), RecordingState::Idle);
        assert!(driver.get_active_region().is_none());
    }
}
