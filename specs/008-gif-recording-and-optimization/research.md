# Research: GIF Recording and Optimization (Feature 008)

## 1. Technical Context & GIF Encoding in Rust

### Image Crate GIF Support
- The `image` crate (v0.25) includes native GIF encoding under the `gif` feature flag:
  ```toml
  image = { version = "0.25", default-features = false, features = ["png", "jpeg", "gif"] }
  ```
- `image::codecs::gif::GifEncoder` supports encoding multiple frames with delay:
  ```rust
  use image::codecs::gif::{GifEncoder, Repeat};
  use image::{Delay, Frame, RgbaImage};

  let mut encoder = GifEncoder::new(&mut output_buffer);
  encoder.set_repeat(Repeat::Infinite).unwrap();
  for frame_buffer in frames {
      let frame = Frame::from_parts(frame_buffer, 0, 0, Delay::from_numer_denom_ms(delay_ms, 1));
      encoder.encode_frame(frame).unwrap();
  }
  ```

---

## 2. Optimization Strategy: Size vs Quality

1. **Framerate Limiting**:
   - Web video is 30-60 FPS; GIFs at 60 FPS create massive files (> 20 MB).
   - Sampling at **12-16 FPS** provides smooth UI animations while reducing frame count by 60-75%.
2. **Color Quantization**:
   - GIF format supports at most 256 colors per palette.
   - The `image` crate quantizes RGBA to 256 colors per frame automatically.
3. **Time Duration Cap**:
   - Cap GIF recordings at **15 seconds maximum** to prevent memory exhaustion and respect corporate chat upload limits.

---

## 3. Naming Convention & Directory

- File name: `Animacion_YYYY-MM-DD_HH-mm-ss.gif`.
- Stored in the user's configured storage directory (or OS standard Pictures directory).
