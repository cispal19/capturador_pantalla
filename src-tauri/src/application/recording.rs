use crate::domain::recording::{RecordingMetadata, RecordingRegion, RecordingState};
use crate::ports::IScreenRecorderDriver;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordingStatusResponse {
    pub state: RecordingState,
    pub elapsed_seconds: u32,
    pub region: Option<RecordingRegion>,
}

#[derive(Clone)]
pub struct RecordingSessionManager {
    driver: Arc<dyn IScreenRecorderDriver>,
    active_region: Arc<Mutex<Option<RecordingRegion>>>,
    last_resume_time: Arc<Mutex<Option<Instant>>>,
    accumulated_seconds: Arc<Mutex<u32>>,
}

impl RecordingSessionManager {
    pub fn new(driver: Arc<dyn IScreenRecorderDriver>) -> Self {
        Self {
            driver,
            active_region: Arc::new(Mutex::new(None)),
            last_resume_time: Arc::new(Mutex::new(None)),
            accumulated_seconds: Arc::new(Mutex::new(0)),
        }
    }

    pub fn start_recording(&self, region: RecordingRegion) -> Result<RecordingRegion, String> {
        self.driver.start(&region).map_err(|e| e.to_string())?;

        *self.active_region.lock().unwrap() = Some(region.clone());
        *self.accumulated_seconds.lock().unwrap() = 0;
        *self.last_resume_time.lock().unwrap() = Some(Instant::now());

        Ok(region)
    }

    pub fn pause_recording(&self) -> Result<(), String> {
        self.driver.pause().map_err(|e| e.to_string())?;

        let mut last_resume = self.last_resume_time.lock().unwrap();
        if let Some(instant) = *last_resume {
            let elapsed = instant.elapsed().as_secs() as u32;
            let mut acc = self.accumulated_seconds.lock().unwrap();
            *acc += elapsed;
            *last_resume = None;
        }

        Ok(())
    }

    pub fn resume_recording(&self) -> Result<(), String> {
        self.driver.resume().map_err(|e| e.to_string())?;

        let mut last_resume = self.last_resume_time.lock().unwrap();
        *last_resume = Some(Instant::now());

        Ok(())
    }

    pub fn stop_recording(
        &self,
        video_bytes: &[u8],
        reported_duration: u32,
        format: &str,
    ) -> Result<RecordingMetadata, String> {
        let meta = self
            .driver
            .stop(video_bytes, reported_duration, format)
            .map_err(|e| e.to_string())?;

        *self.active_region.lock().unwrap() = None;
        *self.last_resume_time.lock().unwrap() = None;
        *self.accumulated_seconds.lock().unwrap() = 0;

        Ok(meta)
    }

    pub fn cancel_recording(&self) -> Result<(), String> {
        self.driver.cancel().map_err(|e| e.to_string())?;

        *self.active_region.lock().unwrap() = None;
        *self.last_resume_time.lock().unwrap() = None;
        *self.accumulated_seconds.lock().unwrap() = 0;

        Ok(())
    }

    pub fn get_state(&self) -> RecordingState {
        self.driver.state()
    }

    pub fn get_status(&self) -> RecordingStatusResponse {
        let state = self.driver.state();
        let region = self.active_region.lock().unwrap().clone();

        let mut total_secs = *self.accumulated_seconds.lock().unwrap();
        if state == RecordingState::Recording {
            if let Some(instant) = *self.last_resume_time.lock().unwrap() {
                total_secs += instant.elapsed().as_secs() as u32;
            }
        }

        RecordingStatusResponse {
            state,
            elapsed_seconds: total_secs,
            region,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::infrastructure::MockScreenRecorderDriver;

    #[test]
    fn test_recording_session_manager_lifecycle() {
        let driver = Arc::new(MockScreenRecorderDriver::new(None));
        let manager = RecordingSessionManager::new(driver);

        assert_eq!(manager.get_state(), RecordingState::Idle);

        let region = RecordingRegion::new(50, 50, 400, 300, 30, 600).unwrap();
        let started_region = manager.start_recording(region.clone()).unwrap();
        assert_eq!(started_region, region);
        assert_eq!(manager.get_state(), RecordingState::Recording);

        let status = manager.get_status();
        assert_eq!(status.state, RecordingState::Recording);
        assert_eq!(status.region, Some(region));

        // Pause
        manager.pause_recording().unwrap();
        assert_eq!(manager.get_state(), RecordingState::Paused);

        // Resume
        manager.resume_recording().unwrap();
        assert_eq!(manager.get_state(), RecordingState::Recording);

        // Stop
        let bytes = vec![1, 2, 3, 4, 5];
        let meta = manager.stop_recording(&bytes, 10, "webm").unwrap();
        assert_eq!(meta.duration_seconds, 10);
        assert_eq!(meta.file_size_bytes, 5);
        assert_eq!(manager.get_state(), RecordingState::Idle);
        assert!(manager.get_status().region.is_none());
    }

    #[test]
    fn test_recording_session_manager_cancel() {
        let driver = Arc::new(MockScreenRecorderDriver::new(None));
        let manager = RecordingSessionManager::new(driver);

        let region = RecordingRegion::new(100, 100, 200, 200, 30, 300).unwrap();
        manager.start_recording(region).unwrap();
        assert_eq!(manager.get_state(), RecordingState::Recording);

        manager.cancel_recording().unwrap();
        assert_eq!(manager.get_state(), RecordingState::Idle);
        assert!(manager.get_status().region.is_none());
    }
}
