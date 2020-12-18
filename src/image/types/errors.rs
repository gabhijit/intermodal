use std::error::Error;
use std::fmt;

pub type ImageResult<T> = Result<T, ImageError>;

#[derive(Debug)]
pub enum ImageError {
    GenericError,
    ReferenceError,
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ImageError::GenericError => write!(f, "GenericError"),
            ImageError::ReferenceError => write!(f, "ReferenceError"),
        }
    }
}
impl Error for ImageError {}
