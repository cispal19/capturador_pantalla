use serde::{Deserialize, Serialize};

/// 8 standard resize handles around a rectangular selection box.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResizeHandle {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl ResizeHandle {
    /// Returns all 8 handles in standard clockwise order starting from NorthWest.
    pub fn all() -> [ResizeHandle; 8] {
        [
            ResizeHandle::NorthWest,
            ResizeHandle::North,
            ResizeHandle::NorthEast,
            ResizeHandle::East,
            ResizeHandle::SouthEast,
            ResizeHandle::South,
            ResizeHandle::SouthWest,
            ResizeHandle::West,
        ]
    }

    /// Returns the standard CSS cursor property for this resize handle.
    pub fn cursor_style(&self) -> &'static str {
        match self {
            ResizeHandle::NorthWest | ResizeHandle::SouthEast => "nwse-resize",
            ResizeHandle::NorthEast | ResizeHandle::SouthWest => "nesw-resize",
            ResizeHandle::North | ResizeHandle::South => "ns-resize",
            ResizeHandle::East | ResizeHandle::West => "ew-resize",
        }
    }

    /// Computes the exact (x, y) anchor coordinate on a bounding box.
    pub fn compute_anchor(&self, x: f64, y: f64, width: f64, height: f64) -> (f64, f64) {
        let half_w = width / 2.0;
        let half_h = height / 2.0;

        match self {
            ResizeHandle::NorthWest => (x, y),
            ResizeHandle::North => (x + half_w, y),
            ResizeHandle::NorthEast => (x + width, y),
            ResizeHandle::East => (x + width, y + half_h),
            ResizeHandle::SouthEast => (x + width, y + height),
            ResizeHandle::South => (x + half_w, y + height),
            ResizeHandle::SouthWest => (x, y + height),
            ResizeHandle::West => (x, y + half_h),
        }
    }
}

/// Anchor descriptor containing handle type, coordinates, and CSS cursor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandleAnchor {
    pub handle: ResizeHandle,
    pub x: f64,
    pub y: f64,
    pub cursor: String,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_resize_handle_count_and_cursor_styles() {
        let handles = ResizeHandle::all();
        assert_eq!(handles.len(), 8);

        assert_eq!(ResizeHandle::NorthWest.cursor_style(), "nwse-resize");
        assert_eq!(ResizeHandle::SouthEast.cursor_style(), "nwse-resize");
        assert_eq!(ResizeHandle::NorthEast.cursor_style(), "nesw-resize");
        assert_eq!(ResizeHandle::SouthWest.cursor_style(), "nesw-resize");
        assert_eq!(ResizeHandle::North.cursor_style(), "ns-resize");
        assert_eq!(ResizeHandle::South.cursor_style(), "ns-resize");
        assert_eq!(ResizeHandle::East.cursor_style(), "ew-resize");
        assert_eq!(ResizeHandle::West.cursor_style(), "ew-resize");
    }

    #[test]
    fn test_handle_anchor_computations() {
        let (x, y, w, h) = (100.0, 200.0, 400.0, 300.0);

        assert_eq!(
            ResizeHandle::NorthWest.compute_anchor(x, y, w, h),
            (100.0, 200.0)
        );
        assert_eq!(
            ResizeHandle::North.compute_anchor(x, y, w, h),
            (300.0, 200.0)
        );
        assert_eq!(
            ResizeHandle::NorthEast.compute_anchor(x, y, w, h),
            (500.0, 200.0)
        );
        assert_eq!(
            ResizeHandle::East.compute_anchor(x, y, w, h),
            (500.0, 350.0)
        );
        assert_eq!(
            ResizeHandle::SouthEast.compute_anchor(x, y, w, h),
            (500.0, 500.0)
        );
        assert_eq!(
            ResizeHandle::South.compute_anchor(x, y, w, h),
            (300.0, 500.0)
        );
        assert_eq!(
            ResizeHandle::SouthWest.compute_anchor(x, y, w, h),
            (100.0, 500.0)
        );
        assert_eq!(
            ResizeHandle::West.compute_anchor(x, y, w, h),
            (100.0, 350.0)
        );
    }
}
