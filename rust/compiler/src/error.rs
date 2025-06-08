use image::{DynamicImage, ImageError};
use miniserde::Deserialize;
use std::error::Error;
use std::fmt;

/// Custom error type to wrap possible failures from `compile_font`
#[derive(Debug)]
pub enum CompileFontError {
    Image(ImageError),
    Json(miniserde::Error),
    PNG(String),
    // You can add more as needed
}

impl fmt::Display for CompileFontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileFontError::Image(e) => write!(f, "Font image error: {}", e),
            CompileFontError::Json(e) => write!(f, "JSON error: {}", e),
            CompileFontError::PNG(e) => write!(f, "Font Sample PNG generation error: {}", e),
        }
    }
}

impl Error for CompileFontError {}

impl From<ImageError> for CompileFontError {
    fn from(e: ImageError) -> Self {
        CompileFontError::Image(e)
    }
}

impl From<miniserde::Error> for CompileFontError {
    fn from(e: miniserde::Error) -> Self {
        CompileFontError::Json(e)
    }
}
 