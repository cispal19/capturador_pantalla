# Data Model: Screen Recording Engine (Feature 007)

## 1. Domain Entities & Value Objects (Rust & TypeScript)

### Rust Domain: `RecordingState`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingState {
    Idle,
    Recording,
    Paused,
    Finalizing,
}
```

### Rust Domain: `RecordingRegion`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordingRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub time_limit_seconds: u32,
}

impl RecordingRegion {
    pub fn new(x: u32, y: u32, width: u32, height: u32, fps: u32, time_limit_seconds: u32) -> Result<Self, String> {
        if width < 50 || height < 50 {
            return Err("Las dimensiones mínimas de grabación son 50x50 píxeles".to_string());
        }

        // Even dimensions required for video containers
        let even_width = width & !1;
        let even_height = height & !1;
        let safe_fps = if fps == 0 { 30 } else { fps.min(60) };
        let safe_limit = if time_limit_seconds == 0 { 600 } else { time_limit_seconds.min(3600) };

        Ok(Self {
            x,
            y,
            width: even_width,
            height: even_height,
            fps: safe_fps,
            time_limit_seconds: safe_limit,
        })
    }
}
```

### Rust Domain: `RecordingMetadata`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub file_path: String,
    pub duration_seconds: u32,
    pub file_size_bytes: usize,
    pub format: String,
    pub fps: u32,
}
```

---

### TypeScript Interfaces

```typescript
export type RecordingState = "Idle" | "Recording" | "Paused" | "Finalizing";

export interface RecordingRegion {
  x: number;
  y: number;
  width: number;
  height: number;
  fps: number;
  timeLimitSeconds: number;
}

export interface RecordingMetadata {
  filePath: string;
  durationSeconds: number;
  fileSizeBytes: number;
  format: string;
  fps: number;
}
```
