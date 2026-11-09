use super::error::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub id: u32,
    pub name: String,
    pub logical_width: u32,
    pub logical_height: u32,
    pub physical_width: u32,
    pub physical_height: u32,
    pub scale_factor: f64,
    pub is_primary: bool,
}

impl DisplayInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u32,
        name: String,
        logical_width: u32,
        logical_height: u32,
        physical_width: u32,
        physical_height: u32,
        scale_factor: f64,
        is_primary: bool,
    ) -> Result<Self, DomainError> {
        if logical_width == 0 || logical_height == 0 || physical_width == 0 || physical_height == 0
        {
            return Err(DomainError::InvalidDimensions {
                width: logical_width as f64,
                height: logical_height as f64,
            });
        }
        if !scale_factor.is_finite() || scale_factor <= 0.0 {
            return Err(DomainError::InvalidScaleFactor(scale_factor));
        }

        Ok(Self {
            id,
            name,
            logical_width,
            logical_height,
            physical_width,
            physical_height,
            scale_factor,
            is_primary,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_display_info() {
        let display = DisplayInfo::new(
            0,
            "Built-in Retina Display".to_string(),
            1728,
            1117,
            3456,
            2234,
            2.0,
            true,
        )
        .unwrap();

        assert_eq!(display.id, 0);
        assert_eq!(display.logical_width, 1728);
        assert_eq!(display.physical_width, 3456);
        assert_eq!(display.scale_factor, 2.0);
        assert!(display.is_primary);
    }

    #[test]
    fn test_reject_zero_dimensions() {
        assert!(DisplayInfo::new(
            1,
            "Invalid Display".to_string(),
            0,
            1080,
            0,
            1080,
            1.0,
            false,
        )
        .is_err());
    }

    #[test]
    fn test_reject_invalid_scale_factor() {
        assert!(DisplayInfo::new(
            1,
            "Invalid Scale Display".to_string(),
            1920,
            1080,
            1920,
            1080,
            0.0,
            false,
        )
        .is_err());

        assert!(DisplayInfo::new(
            1,
            "Invalid Scale Display".to_string(),
            1920,
            1080,
            1920,
            1080,
            -1.5,
            false,
        )
        .is_err());
    }
}
