use super::error::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

impl Dimensions {
    pub fn new(width: f64, height: f64) -> Result<Self, DomainError> {
        if !width.is_finite() || !height.is_finite() || width < 0.0 || height < 0.0 {
            return Err(DomainError::InvalidDimensions { width, height });
        }
        Ok(Self { width, height })
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0.0 || self.height == 0.0
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_dimensions() {
        let dims = Dimensions::new(800.0, 600.0).unwrap();
        assert_eq!(dims.width, 800.0);
        assert_eq!(dims.height, 600.0);
        assert_eq!(dims.area(), 480000.0);
        assert!(!dims.is_empty());
    }

    #[test]
    fn test_reject_negative_or_nan_dimensions() {
        assert!(Dimensions::new(-1.0, 100.0).is_err());
        assert!(Dimensions::new(100.0, -5.0).is_err());
        assert!(Dimensions::new(f64::NAN, 100.0).is_err());
    }

    #[test]
    fn test_empty_dimensions() {
        let zero = Dimensions::new(0.0, 50.0).unwrap();
        assert!(zero.is_empty());
    }
}
