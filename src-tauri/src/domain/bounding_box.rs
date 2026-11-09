use super::{
    coordinates::DisplayCoordinates,
    dimensions::Dimensions,
    error::DomainError,
    handle::{HandleAnchor, ResizeHandle},
};
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

    /// Generates the 8 standard handle anchors for this bounding box
    pub fn handle_anchors(&self) -> Vec<HandleAnchor> {
        ResizeHandle::all()
            .iter()
            .map(|handle| {
                let (x, y) = handle.compute_anchor(
                    self.origin.x,
                    self.origin.y,
                    self.size.width,
                    self.size.height,
                );
                HandleAnchor {
                    handle: *handle,
                    x,
                    y,
                    cursor: handle.cursor_style().to_string(),
                }
            })
            .collect()
    }

    /// Hit-tests whether a given coordinate is within tolerance radius of any handle
    pub fn hit_test_handle(&self, px: f64, py: f64, radius: f64) -> Option<ResizeHandle> {
        for anchor in self.handle_anchors() {
            if (px - anchor.x).abs() <= radius && (py - anchor.y).abs() <= radius {
                return Some(anchor.handle);
            }
        }
        None
    }

    /// Determines if a point is within the interior bounds of the bounding box
    pub fn contains_point(&self, px: f64, py: f64) -> bool {
        px >= self.origin.x
            && px <= (self.origin.x + self.size.width)
            && py >= self.origin.y
            && py <= (self.origin.y + self.size.height)
    }

    /// Translates the bounding box origin by (dx, dy), clamped to [0, max_width] x [0, max_height]
    pub fn translate(
        &self,
        dx: f64,
        dy: f64,
        max_width: f64,
        max_height: f64,
    ) -> Result<Self, DomainError> {
        let max_x = (max_width - self.size.width).max(0.0);
        let max_y = (max_height - self.size.height).max(0.0);

        let new_x = (self.origin.x + dx).max(0.0).min(max_x);
        let new_y = (self.origin.y + dy).max(0.0).min(max_y);

        let origin = DisplayCoordinates::new(new_x, new_y)?;
        Self::new(origin, self.size, self.scale_factor)
    }

    /// Resizes the bounding box by moving a specific handle anchor by (dx, dy), respecting minimum size
    pub fn resize_by_handle(
        &self,
        handle: ResizeHandle,
        dx: f64,
        dy: f64,
        min_size: f64,
    ) -> Result<Self, DomainError> {
        let mut x = self.origin.x;
        let mut y = self.origin.y;
        let mut w = self.size.width;
        let mut h = self.size.height;

        match handle {
            ResizeHandle::NorthWest => {
                let new_w = (w - dx).max(min_size);
                let actual_dx = w - new_w;
                x += actual_dx;
                w = new_w;

                let new_h = (h - dy).max(min_size);
                let actual_dy = h - new_h;
                y += actual_dy;
                h = new_h;
            }
            ResizeHandle::North => {
                let new_h = (h - dy).max(min_size);
                let actual_dy = h - new_h;
                y += actual_dy;
                h = new_h;
            }
            ResizeHandle::NorthEast => {
                w = (w + dx).max(min_size);
                let new_h = (h - dy).max(min_size);
                let actual_dy = h - new_h;
                y += actual_dy;
                h = new_h;
            }
            ResizeHandle::East => {
                w = (w + dx).max(min_size);
            }
            ResizeHandle::SouthEast => {
                w = (w + dx).max(min_size);
                h = (h + dy).max(min_size);
            }
            ResizeHandle::South => {
                h = (h + dy).max(min_size);
            }
            ResizeHandle::SouthWest => {
                let new_w = (w - dx).max(min_size);
                let actual_dx = w - new_w;
                x += actual_dx;
                w = new_w;
                h = (h + dy).max(min_size);
            }
            ResizeHandle::West => {
                let new_w = (w - dx).max(min_size);
                let actual_dx = w - new_w;
                x += actual_dx;
                w = new_w;
            }
        }

        let origin = DisplayCoordinates::new(x, y)?;
        let size = Dimensions::new(w, h)?;
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
    fn test_handle_anchors_generation() {
        let origin = DisplayCoordinates::new(100.0, 100.0).unwrap();
        let size = Dimensions::new(200.0, 100.0).unwrap();
        let bbox = BoundingBox::new(origin, size, 1.0).unwrap();

        let anchors = bbox.handle_anchors();
        assert_eq!(anchors.len(), 8);

        let nw = anchors
            .iter()
            .find(|a| a.handle == ResizeHandle::NorthWest)
            .unwrap();
        assert_eq!((nw.x, nw.y), (100.0, 100.0));

        let se = anchors
            .iter()
            .find(|a| a.handle == ResizeHandle::SouthEast)
            .unwrap();
        assert_eq!((se.x, se.y), (300.0, 200.0));
    }

    #[test]
    fn test_hit_test_handle_and_contains_point() {
        let origin = DisplayCoordinates::new(100.0, 100.0).unwrap();
        let size = Dimensions::new(200.0, 100.0).unwrap();
        let bbox = BoundingBox::new(origin, size, 1.0).unwrap();

        // Exact hit on SouthEast (300, 200) with tolerance 5.0
        assert_eq!(
            bbox.hit_test_handle(299.0, 201.0, 5.0),
            Some(ResizeHandle::SouthEast)
        );
        // Miss hit
        assert_eq!(bbox.hit_test_handle(150.0, 150.0, 5.0), None);

        // Inside point
        assert!(bbox.contains_point(150.0, 150.0));
        // Outside point
        assert!(!bbox.contains_point(50.0, 50.0));
    }

    #[test]
    fn test_translate_with_clamping() {
        let origin = DisplayCoordinates::new(100.0, 100.0).unwrap();
        let size = Dimensions::new(200.0, 100.0).unwrap();
        let bbox = BoundingBox::new(origin, size, 1.0).unwrap();

        // Translate +50, +30 within 1920x1080
        let moved = bbox.translate(50.0, 30.0, 1920.0, 1080.0).unwrap();
        assert_eq!(moved.origin.x, 150.0);
        assert_eq!(moved.origin.y, 130.0);
        assert_eq!(moved.size.width, 200.0);
        assert_eq!(moved.size.height, 100.0);

        // Translate beyond right edge
        let pushed_right = bbox.translate(2000.0, 0.0, 1920.0, 1080.0).unwrap();
        assert_eq!(pushed_right.origin.x, 1720.0); // 1920 - 200
    }

    #[test]
    fn test_resize_by_handle_southeast_and_northwest() {
        let origin = DisplayCoordinates::new(100.0, 100.0).unwrap();
        let size = Dimensions::new(200.0, 100.0).unwrap();
        let bbox = BoundingBox::new(origin, size, 1.0).unwrap();

        // Resize SE: dx = +50, dy = +20
        let resized_se = bbox
            .resize_by_handle(ResizeHandle::SouthEast, 50.0, 20.0, 10.0)
            .unwrap();
        assert_eq!(resized_se.origin.x, 100.0);
        assert_eq!(resized_se.origin.y, 100.0);
        assert_eq!(resized_se.size.width, 250.0);
        assert_eq!(resized_se.size.height, 120.0);

        // Resize NW: dx = +20, dy = +10
        let resized_nw = bbox
            .resize_by_handle(ResizeHandle::NorthWest, 20.0, 10.0, 10.0)
            .unwrap();
        assert_eq!(resized_nw.origin.x, 120.0);
        assert_eq!(resized_nw.origin.y, 110.0);
        assert_eq!(resized_nw.size.width, 180.0);
        assert_eq!(resized_nw.size.height, 90.0);

        // Resize NW below min_size (10.0)
        let shrunk_nw = bbox
            .resize_by_handle(ResizeHandle::NorthWest, 300.0, 300.0, 10.0)
            .unwrap();
        assert_eq!(shrunk_nw.size.width, 10.0);
        assert_eq!(shrunk_nw.size.height, 10.0);
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
