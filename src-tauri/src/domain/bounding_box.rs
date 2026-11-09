use super::{coordinates::DisplayCoordinates, dimensions::Dimensions, error::DomainError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub origin: DisplayCoordinates,
    pub size: Dimensions,
    pub scale_factor: f64,
}

impl BoundingBox {
    pub fn new(
        origin: DisplayCoordinates,
        size: Dimensions,
        scale_factor: f64,
    ) -> Result<Self, DomainError> {
        if !scale_factor.is_finite() || scale_factor <= 0.0 {
            return Err(DomainError::InvalidScaleFactor(scale_factor));
        }
        Ok(Self {
            origin,
            size,
            scale_factor,
        })
    }

    /// Normalizes arbitrary drag points (e.g. dragged bottom-right to top-left)
    pub fn from_points(
        p1: DisplayCoordinates,
        p2: DisplayCoordinates,
        scale_factor: f64,
    ) -> Result<Self, DomainError> {
        if !scale_factor.is_finite() || scale_factor <= 0.0 {
            return Err(DomainError::InvalidScaleFactor(scale_factor));
        }

        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let width = (p2.x - p1.x).abs();
        let height = (p2.y - p1.y).abs();

        let origin = DisplayCoordinates::new(min_x, min_y)?;
        let size = Dimensions::new(width, height)?;

        Ok(Self {
            origin,
            size,
            scale_factor,
        })
    }

    /// Clamps the bounding box to lie strictly within (0, 0, max_w, max_h)
    pub fn clamp(&self, max_width: f64, max_height: f64) -> Result<Self, DomainError> {
        let clamped_x = self.origin.x.max(0.0).min(max_width);
        let clamped_y = self.origin.y.max(0.0).min(max_height);

        let available_w = (max_width - clamped_x).max(0.0);
        let available_h = (max_height - clamped_y).max(0.0);

        let clamped_w = self.size.width.min(available_w);
        let clamped_h = self.size.height.min(available_h);

        let origin = DisplayCoordinates::new(clamped_x, clamped_y)?;
        let size = Dimensions::new(clamped_w, clamped_h)?;

        Self::new(origin, size, self.scale_factor)
    }

    /// Converts logical coordinates to physical pixels taking scale_factor into account
    pub fn to_physical_pixels(&self) -> (u32, u32, u32, u32) {
        let px = (self.origin.x * self.scale_factor).round().max(0.0) as u32;
        let py = (self.origin.y * self.scale_factor).round().max(0.0) as u32;
        let pw = (self.size.width * self.scale_factor).round().max(0.0) as u32;
        let ph = (self.size.height * self.scale_factor).round().max(0.0) as u32;
        (px, py, pw, ph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_inverted_drag_normalization() {
        // Dragged from bottom-right (500, 400) to top-left (100, 50)
        let p1 = DisplayCoordinates::new(500.0, 400.0).unwrap();
        let p2 = DisplayCoordinates::new(100.0, 50.0).unwrap();

        let bbox = BoundingBox::from_points(p1, p2, 2.0).unwrap();

        assert_eq!(bbox.origin.x, 100.0);
        assert_eq!(bbox.origin.y, 50.0);
        assert_eq!(bbox.size.width, 400.0);
        assert_eq!(bbox.size.height, 350.0);
        assert_eq!(bbox.scale_factor, 2.0);

        let (px, py, pw, ph) = bbox.to_physical_pixels();
        assert_eq!(px, 200);
        assert_eq!(py, 100);
        assert_eq!(pw, 800);
        assert_eq!(ph, 700);
    }

    #[test]
    fn test_bounding_box_clamping() {
        let p1 = DisplayCoordinates::new(-50.0, -20.0).unwrap();
        let p2 = DisplayCoordinates::new(2000.0, 1500.0).unwrap();
        let bbox = BoundingBox::from_points(p1, p2, 1.0).unwrap();

        let clamped = bbox.clamp(1920.0, 1080.0).unwrap();

        assert_eq!(clamped.origin.x, 0.0);
        assert_eq!(clamped.origin.y, 0.0);
        assert_eq!(clamped.size.width, 1920.0);
        assert_eq!(clamped.size.height, 1080.0);
    }

    #[test]
    fn test_reject_invalid_scale_factor() {
        let p1 = DisplayCoordinates::new(0.0, 0.0).unwrap();
        let p2 = DisplayCoordinates::new(10.0, 10.0).unwrap();
        assert!(BoundingBox::from_points(p1, p2, 0.0).is_err());
        assert!(BoundingBox::from_points(p1, p2, -1.5).is_err());
        assert!(BoundingBox::from_points(p1, p2, f64::NAN).is_err());
    }
}
