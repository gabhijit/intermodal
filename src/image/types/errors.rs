use std::error::Error;
use std::fmt;

pub type ImageResult<T> = Result<T, ImageError>;

/// Error object related to Image Handling.
#[derive(Debug)]
pub enum ImageError {
    /// A placeholder for all not-yet qualified Errors.
    GenericError,
    /// Error related to Parsing underlying Image Reference
    ReferenceError,
    /// Input Image format not confirming to [transport:reference]
    ParseError,
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ImageError::GenericError => write!(f, "GenericError"),
            ImageError::ReferenceError => write!(f, "ReferenceError"),
            ImageError::ParseError => write!(f, "ParseError"),
        }
    }
}
impl Error for ImageError {}
