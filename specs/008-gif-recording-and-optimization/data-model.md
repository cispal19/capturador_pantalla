# Data Model: GIF Recording and Optimization (Feature 008)

## 1. Domain Entities & Value Objects (Rust & TypeScript)

### Rust Domain: `GifExportConfig`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GifExportConfig {
    pub fps: u32,
    pub max_duration_seconds: u32,
    pub repeat_infinite: bool,
}

impl Default for GifExportConfig {
    fn default() -> Self {
        Self {
            fps: 15,
            max_duration_seconds: 15,
            repeat_infinite: true,
        }
    }
}

impl GifExportConfig {
    pub fn new(fps: u32, max_duration_seconds: u32) -> Self {
        let safe_fps = fps.clamp(5, 24);
        let safe_duration = max_duration_seconds.clamp(1, 15);
        Self {
            fps: safe_fps,
            max_duration_seconds: safe_duration,
            repeat_infinite: true,
        }
    }

    pub fn frame_delay_ms(&self) -> u32 {
        1000 / self.fps.max(1)
    }
}
```

### Rust Domain: `GifExportResult`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GifExportResult {
    pub file_path: String,
    pub duration_seconds: u32,
    pub file_size_bytes: usize,
    pub frame_count: usize,
}
```

---

### TypeScript Interfaces

```typescript
export interface GifExportConfig {
  fps: number;
  maxDurationSeconds: number;
  repeatInfinite: boolean;
}

export interface GifExportResult {
  filePath: string;
  durationSeconds: number;
  fileSizeBytes: number;
  frameCount: number;
}
```
