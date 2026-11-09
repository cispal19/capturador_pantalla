use crate::domain::gif::{GifExportConfig, GifExportResult};
use crate::domain::naming::FileNameGenerator;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, RgbaImage};
use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use std::time::SystemTime;

pub struct ExportRecordingToGifUseCase;

impl ExportRecordingToGifUseCase {
    pub fn execute(
        width: u32,
        height: u32,
        frames: Vec<Vec<u8>>,
        config: GifExportConfig,
        target_dir: Option<PathBuf>,
    ) -> Result<GifExportResult, String> {
        if frames.is_empty() {
            return Err("Se requiere al menos un fotograma para generar un GIF".to_string());
        }
        if width == 0 || height == 0 {
            return Err("Las dimensiones del GIF no pueden ser cero".to_string());
        }

        let expected_frame_len = (width as usize) * (height as usize) * 4;
        for (i, frame_data) in frames.iter().enumerate() {
            if frame_data.len() != expected_frame_len {
                return Err(format!(
                    "Tamaño de buffer inválido en fotograma {}: esperado {} bytes, obtenido {}",
                    i,
                    expected_frame_len,
                    frame_data.len()
                ));
            }
        }

        let dir = target_dir.unwrap_or_else(env::temp_dir);
        let _ = fs::create_dir_all(&dir);

        let now_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let file_name = FileNameGenerator::generate_animation_name("gif", now_secs);
        let file_path = dir.join(file_name);

        let file = File::create(&file_path).map_err(|e| e.to_string())?;
        let mut encoder = GifEncoder::new(file);
        if config.repeat_infinite {
            let _ = encoder.set_repeat(Repeat::Infinite);
        }

        let delay_ms = config.frame_delay_ms();
        let delay = Delay::from_numer_denom_ms(delay_ms, 1);

        let frame_count = frames.len();
        for frame_raw in frames {
            let img = RgbaImage::from_raw(width, height, frame_raw)
                .ok_or_else(|| "Error al construir imagen RGBA para fotograma".to_string())?;
            let frame = Frame::from_parts(img, 0, 0, delay);
            encoder.encode_frame(frame).map_err(|e| e.to_string())?;
        }

        let size = fs::metadata(&file_path)
            .map(|m| m.len() as usize)
            .unwrap_or(0);
        let duration_seconds = (frame_count as u32) / config.fps.max(1);

        Ok(GifExportResult {
            file_path: file_path.to_string_lossy().to_string(),
            duration_seconds: duration_seconds.max(1),
            file_size_bytes: size,
            frame_count,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_export_recording_to_gif_use_case_valid() {
        let temp_dir = env::temp_dir().join("test_gif_export");
        let _ = fs::remove_dir_all(&temp_dir);

        let w = 20;
        let h = 20;
        let frame1 = vec![255, 0, 0, 255].repeat((w * h) as usize); // Red
        let frame2 = vec![0, 255, 0, 255].repeat((w * h) as usize); // Green
        let frame3 = vec![0, 0, 255, 255].repeat((w * h) as usize); // Blue

        let config = GifExportConfig::new(10, 5);
        let res = ExportRecordingToGifUseCase::execute(
            w,
            h,
            vec![frame1, frame2, frame3],
            config,
            Some(temp_dir.clone()),
        )
        .unwrap();

        assert_eq!(res.frame_count, 3);
        assert!(res.file_size_bytes > 0);
        assert!(std::path::Path::new(&res.file_path).exists());

        // Validate GIF89a header magic bytes
        let bytes = fs::read(&res.file_path).unwrap();
        assert!(bytes.starts_with(b"GIF89a"));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_export_recording_to_gif_empty_frames() {
        let config = GifExportConfig::default();
        assert!(ExportRecordingToGifUseCase::execute(10, 10, vec![], config, None).is_err());
    }

    #[test]
    fn test_export_recording_to_gif_mismatch_buffer() {
        let config = GifExportConfig::default();
        let invalid_frame = vec![0; 50]; // Not 10x10x4 = 400
        assert!(
            ExportRecordingToGifUseCase::execute(10, 10, vec![invalid_frame], config, None)
                .is_err()
        );
    }
}
