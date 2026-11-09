/// Region coordinates for applying pixelation obfuscation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObfuscationRegion {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl ObfuscationRegion {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Domain service implementing irreversible block-averaging pixelation for sensitive data.
pub struct ObfuscationKernel;

impl ObfuscationKernel {
    /// Applies irreversible block-averaging pixelation to an RGBA pixel buffer region in-place.
    ///
    /// Each block of `block_size x block_size` pixels is flattened to the arithmetic
    /// mean color of its constituents, completely destroying high-frequency textual details.
    pub fn apply_pixelate(
        buffer: &mut [u8],
        buf_width: usize,
        buf_height: usize,
        region: ObfuscationRegion,
        block_size: usize,
    ) {
        if block_size == 0
            || buf_width == 0
            || buf_height == 0
            || region.width == 0
            || region.height == 0
        {
            return;
        }

        let max_x = (region.x + region.width).min(buf_width);
        let max_y = (region.y + region.height).min(buf_height);
        let start_x = region.x.min(buf_width);
        let start_y = region.y.min(buf_height);

        let mut by = start_y;
        while by < max_y {
            let by_end = (by + block_size).min(max_y);

            let mut bx = start_x;
            while bx < max_x {
                let bx_end = (bx + block_size).min(max_x);

                // Pass 1: Compute channel sums
                let mut sum_r: u64 = 0;
                let mut sum_g: u64 = 0;
                let mut sum_b: u64 = 0;
                let mut count: u64 = 0;

                for y in by..by_end {
                    for x in bx..bx_end {
                        let offset = (y * buf_width + x) * 4;
                        if offset + 3 < buffer.len() {
                            sum_r += buffer[offset] as u64;
                            sum_g += buffer[offset + 1] as u64;
                            sum_b += buffer[offset + 2] as u64;
                            count += 1;
                        }
                    }
                }

                if let (Some(r), Some(g), Some(b)) = (
                    sum_r.checked_div(count),
                    sum_g.checked_div(count),
                    sum_b.checked_div(count),
                ) {
                    let avg_r = r as u8;
                    let avg_g = g as u8;
                    let avg_b = b as u8;

                    // Pass 2: Overwrite pixels with flat average
                    for y in by..by_end {
                        for x in bx..bx_end {
                            let offset = (y * buf_width + x) * 4;
                            if offset + 3 < buffer.len() {
                                buffer[offset] = avg_r;
                                buffer[offset + 1] = avg_g;
                                buffer[offset + 2] = avg_b;
                                // Keep alpha channel intact
                            }
                        }
                    }
                }

                bx += block_size;
            }

            by += block_size;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_obfuscation_pixelate_irreversibility() {
        let width = 8;
        let height = 8;
        let mut buffer = vec![0u8; width * height * 4];

        // Fill with alternating black (0, 0, 0) and white (255, 255, 255) pixels (checkerboard)
        for y in 0..height {
            for x in 0..width {
                let offset = (y * width + x) * 4;
                let val = if (x + y) % 2 == 0 { 255 } else { 0 };
                buffer[offset] = val;
                buffer[offset + 1] = val;
                buffer[offset + 2] = val;
                buffer[offset + 3] = 255;
            }
        }

        let region = ObfuscationRegion::new(0, 0, width, height);
        ObfuscationKernel::apply_pixelate(&mut buffer, width, height, region, 4);

        // In a 4x4 block of alternating 255 and 0, exactly 8 pixels are 255 and 8 are 0.
        // Expected mean: (8 * 255) / 16 = 127
        for y in 0..4 {
            for x in 0..4 {
                let offset = (y * width + x) * 4;
                assert_eq!(
                    buffer[offset], 127,
                    "Pixel at ({}, {}) should be flattened to 127",
                    x, y
                );
                assert_eq!(buffer[offset + 1], 127);
                assert_eq!(buffer[offset + 2], 127);
                assert_eq!(buffer[offset + 3], 255);
            }
        }
    }

    #[test]
    fn test_obfuscation_boundary_clamping_no_panic() {
        let mut buffer = vec![100u8; 10 * 10 * 4];

        // Rect extends beyond buffer bounds
        let region = ObfuscationRegion::new(5, 5, 20, 20);
        ObfuscationKernel::apply_pixelate(&mut buffer, 10, 10, region, 4);

        // Ensure buffer still valid length and no panic
        assert_eq!(buffer.len(), 400);
    }
}
