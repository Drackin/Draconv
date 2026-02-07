use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Error)]
pub enum Error {
    #[error("Path is not a file: {0}")]
    NotAFile(String),
    #[error("Invalid file path")]
    InvalidPath,
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Conversion was cancelled by the user")]
    ConversionCancelled,
    #[error("FFmpeg process failed: {0}")]
    FfmpegFailed(String),
    #[error("Could not capture FFmpeg stdout")]
    FfmpegStdout,
    #[error("Image processing error: {0}")]
    ImageError(String),
    #[error("Invalid category for conversion: {0}")]
    InvalidCategory(String),
    #[error("Job not found")]
    JobNotFound,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

//pub type Result<T> = std::result::Result<T, Error>;
