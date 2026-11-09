use crate::domain::{buffer::CaptureSessionState, error::DomainError};

pub struct CaptureSessionManager {
    current_state: CaptureSessionState,
}

impl Default for CaptureSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CaptureSessionManager {
    pub fn new() -> Self {
        Self {
            current_state: CaptureSessionState::Idle,
        }
    }

    pub fn state(&self) -> CaptureSessionState {
        self.current_state
    }

    pub fn transition_to(&mut self, next: CaptureSessionState) -> Result<(), DomainError> {
        let valid = matches!(
            (self.current_state, next),
            (CaptureSessionState::Idle, CaptureSessionState::Selecting)
                | (
                    CaptureSessionState::Selecting,
                    CaptureSessionState::Completed
                )
                | (
                    CaptureSessionState::Selecting,
                    CaptureSessionState::Cancelled
                )
                | (CaptureSessionState::Completed, CaptureSessionState::Idle)
                | (CaptureSessionState::Cancelled, CaptureSessionState::Idle)
        );

        if !valid {
            return Err(DomainError::InvalidStateTransition {
                from: format!("{:?}", self.current_state),
                to: format!("{:?}", next),
            });
        }

        self.current_state = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_session_lifecycle() {
        let mut session = CaptureSessionManager::new();
        assert_eq!(session.state(), CaptureSessionState::Idle);

        session
            .transition_to(CaptureSessionState::Selecting)
            .unwrap();
        assert_eq!(session.state(), CaptureSessionState::Selecting);

        session
            .transition_to(CaptureSessionState::Completed)
            .unwrap();
        assert_eq!(session.state(), CaptureSessionState::Completed);

        session.transition_to(CaptureSessionState::Idle).unwrap();
        assert_eq!(session.state(), CaptureSessionState::Idle);
    }

    #[test]
    fn test_cancellation_lifecycle() {
        let mut session = CaptureSessionManager::new();
        session
            .transition_to(CaptureSessionState::Selecting)
            .unwrap();
        session
            .transition_to(CaptureSessionState::Cancelled)
            .unwrap();
        assert_eq!(session.state(), CaptureSessionState::Cancelled);
        session.transition_to(CaptureSessionState::Idle).unwrap();
    }

    #[test]
    fn test_invalid_transition() {
        let mut session = CaptureSessionManager::new();
        // Cannot jump directly from Idle to Completed
        assert!(session
            .transition_to(CaptureSessionState::Completed)
            .is_err());
    }
}
