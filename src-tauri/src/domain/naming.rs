/// Pure domain generator for timestamped, collision-free screenshot filenames.
pub struct FileNameGenerator;

impl FileNameGenerator {
    /// Formats a filename with standard prefix and timestamp: `Captura_YYYY-MM-DD_HH-mm-ss.{ext}`
    pub fn generate_name(ext: &str, timestamp_secs: u64) -> String {
        let (year, month, day, hour, min, sec) = Self::timestamp_to_datetime(timestamp_secs);
        let clean_ext = ext.trim_start_matches('.');
        format!(
            "Captura_{:04}-{:02}-{:02}_{:02}-{:02}-{:02}.{}",
            year, month, day, hour, min, sec, clean_ext
        )
    }

    /// Pure mathematical decomposition of seconds since Unix Epoch (1970-01-01 00:00:00 UTC)
    fn timestamp_to_datetime(secs: u64) -> (u32, u32, u32, u32, u32, u32) {
        let sec = (secs % 60) as u32;
        let total_mins = secs / 60;
        let min = (total_mins % 60) as u32;
        let total_hours = total_mins / 60;
        let hour = (total_hours % 24) as u32;
        let mut days = (total_hours / 24) as u32;

        // Days since 1970-01-01
        let mut year = 1970;
        loop {
            let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            let days_in_year = if leap { 366 } else { 365 };
            if days >= days_in_year {
                days -= days_in_year;
                year += 1;
            } else {
                break;
            }
        }

        let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_per_month = [
            31,
            if leap { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];

        let mut month = 1;
        for &m_days in &days_per_month {
            if days >= m_days {
                days -= m_days;
                month += 1;
            } else {
                break;
            }
        }
        let day = days + 1;

        (year, month, day, hour, min, sec)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_file_name_generator_known_timestamp() {
        // 2026-11-09 12:00:00 UTC = 1794225600
        let ts = 1794225600;
        let name_png = FileNameGenerator::generate_name("png", ts);
        assert_eq!(name_png, "Captura_2026-11-09_12-00-00.png");

        let name_jpg = FileNameGenerator::generate_name(".jpg", ts);
        assert_eq!(name_jpg, "Captura_2026-11-09_12-00-00.jpg");
    }

    #[test]
    fn test_file_name_generator_epoch() {
        let name = FileNameGenerator::generate_name("webp", 0);
        assert_eq!(name, "Captura_1970-01-01_00-00-00.webp");
    }
}
