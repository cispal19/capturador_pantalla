use super::error::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DisplayCoordinates {
    pub x: f64,
    pub y: f64,
}

impl DisplayCoordinates {
    pub fn new(x: f64, y: f64) -> Result<Self, DomainError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(DomainError::NonFiniteCoordinate(format!(
                "x={}, y={}",
                x, y
            )));
        }
        Ok(Self { x, y })
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Result<Self, DomainError> {
        Self::new(self.x + dx, self.y + dy)
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_coordinates() {
        let coord = DisplayCoordinates::new(100.5, 200.75).unwrap();
        assert_eq!(coord.x, 100.5);
        assert_eq!(coord.y, 200.75);
    }

    #[test]
    fn test_reject_non_finite_coordinates() {
        assert!(DisplayCoordinates::new(f64::NAN, 10.0).is_err());
        assert!(DisplayCoordinates::new(10.0, f64::INFINITY).is_err());
        assert!(DisplayCoordinates::new(f64::NEG_INFINITY, 10.0).is_err());
    }

    #[test]
    fn test_distance_and_translation() {
        let c1 = DisplayCoordinates::new(0.0, 0.0).unwrap();
        let c2 = DisplayCoordinates::new(3.0, 4.0).unwrap();
        assert_eq!(c1.distance_to(&c2), 5.0);

        let translated = c1.translate(10.0, -5.0).unwrap();
        assert_eq!(translated.x, 10.0);
        assert_eq!(translated.y, -5.0);
    }
}
