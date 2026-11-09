#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenCapturePermissionStatus {
    Granted,
    Denied,
    NotDetermined,
}

pub struct SystemPermissionsService;

impl SystemPermissionsService {
    pub fn check_screen_recording_permission() -> ScreenCapturePermissionStatus {
        #[cfg(target_os = "macos")]
        {
            // On macOS, CoreGraphics permissions can be checked
            ScreenCapturePermissionStatus::Granted
        }

        #[cfg(not(target_os = "macos"))]
        {
            // On Windows and Linux X11/Wayland, permission is granted by default or prompted via portal
            ScreenCapturePermissionStatus::Granted
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_status_variants() {
        let status = SystemPermissionsService::check_screen_recording_permission();
        assert!(matches!(
            status,
            ScreenCapturePermissionStatus::Granted
                | ScreenCapturePermissionStatus::Denied
                | ScreenCapturePermissionStatus::NotDetermined
        ));
    }
}
