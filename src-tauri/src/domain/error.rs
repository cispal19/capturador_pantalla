use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("Coordinate value must be finite: {0}")]
    NonFiniteCoordinate(String),

    #[error("Dimension values must be non-negative: width={width}, height={height}")]
    InvalidDimensions { width: f64, height: f64 },

    #[error("Scale factor must be strictly positive: {0}")]
    InvalidScaleFactor(f64),

    #[error("Buffer byte length mismatch: expected {expected}, got {actual}")]
    BufferLengthMismatch { expected: usize, actual: usize },

    #[error("Invalid state transition from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },
}
