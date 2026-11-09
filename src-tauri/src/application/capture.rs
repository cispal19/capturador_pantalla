use super::session::CaptureSessionManager;
use crate::domain::buffer::{CaptureSessionState, RawDisplayBuffer};
use crate::ports::{CaptureDriverError, IScreenCaptureDriver};
use std::sync::{Arc, Mutex};

pub struct TriggerCaptureSessionUseCase {
    session_manager: Arc<Mutex<CaptureSessionManager>>,
    capture_driver: Arc<dyn IScreenCaptureDriver>,
}

impl TriggerCaptureSessionUseCase {
    pub fn new(
        session_manager: Arc<Mutex<CaptureSessionManager>>,
        capture_driver: Arc<dyn IScreenCaptureDriver>,
    ) -> Self {
        Self {
            session_manager,
            capture_driver,
        }
    }

    pub fn execute(
        &self,
        target_display_id: Option<u32>,
    ) -> Result<RawDisplayBuffer, CaptureDriverError> {
        let mut session = self.session_manager.lock().unwrap();
        if session.state() == CaptureSessionState::Selecting {
            return Err(CaptureDriverError::Internal(
                "Capture session already in progress".to_string(),
            ));
        }

        let buffer = match target_display_id {
            Some(id) => self.capture_driver.capture_display_by_id(id)?,
            None => self.capture_driver.capture_active_display()?,
        };

        session
            .transition_to(CaptureSessionState::Selecting)
            .map_err(|e| CaptureDriverError::Internal(e.to_string()))?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{buffer::PixelFormat, display::DisplayInfo};

    struct TestCaptureDriver {
        should_fail: bool,
    }

    impl IScreenCaptureDriver for TestCaptureDriver {
        fn get_displays(&self) -> Result<Vec<DisplayInfo>, CaptureDriverError> {
            Ok(vec![DisplayInfo::new(
                0,
                "Test".into(),
                1920,
                1080,
                1920,
                1080,
                1.0,
                true,
            )
            .unwrap()])
        }

        fn capture_active_display(&self) -> Result<RawDisplayBuffer, CaptureDriverError> {
            if self.should_fail {
                return Err(CaptureDriverError::PermissionDenied);
            }
            RawDisplayBuffer::new(10, 10, 40, PixelFormat::Rgba8, vec![255; 400])
                .map_err(|e| CaptureDriverError::Internal(e.to_string()))
        }

        fn capture_display_by_id(
            &self,
            display_id: u32,
        ) -> Result<RawDisplayBuffer, CaptureDriverError> {
            if display_id == 0 {
                self.capture_active_display()
            } else {
                Err(CaptureDriverError::DisplayNotFound(display_id))
            }
        }

        fn capture_all_displays(
            &self,
        ) -> Result<Vec<(DisplayInfo, RawDisplayBuffer)>, CaptureDriverError> {
            let info = self.get_displays()?.remove(0);
            let buf = self.capture_active_display()?;
            Ok(vec![(info, buf)])
        }
    }

    #[test]
    fn test_trigger_session_success_and_state_transition() {
        let manager = Arc::new(Mutex::new(CaptureSessionManager::new()));
        let driver = Arc::new(TestCaptureDriver { should_fail: false });
        let use_case = TriggerCaptureSessionUseCase::new(manager.clone(), driver);

        assert_eq!(manager.lock().unwrap().state(), CaptureSessionState::Idle);

        let result = use_case.execute(None);
        assert!(result.is_ok());
        assert_eq!(
            manager.lock().unwrap().state(),
            CaptureSessionState::Selecting
        );
    }

    #[test]
    fn test_trigger_session_debounce_when_already_selecting() {
        let manager = Arc::new(Mutex::new(CaptureSessionManager::new()));
        let driver = Arc::new(TestCaptureDriver { should_fail: false });
        let use_case = TriggerCaptureSessionUseCase::new(manager.clone(), driver);

        assert!(use_case.execute(None).is_ok());
        // Second immediate call while Selecting should be rejected (debounced)
        let second = use_case.execute(None);
        assert!(second.is_err());
    }

    #[test]
    fn test_trigger_session_driver_error_keeps_idle_state() {
        let manager = Arc::new(Mutex::new(CaptureSessionManager::new()));
        let driver = Arc::new(TestCaptureDriver { should_fail: true });
        let use_case = TriggerCaptureSessionUseCase::new(manager.clone(), driver);

        let result = use_case.execute(None);
        assert!(matches!(result, Err(CaptureDriverError::PermissionDenied)));
        assert_eq!(manager.lock().unwrap().state(), CaptureSessionState::Idle);
    }
}
