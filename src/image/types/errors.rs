use std::error::Error as StdError;
use std::fmt;

pub type ImageResult<T> = Result<T, ImageError>;

pub(crate) type Cause = Box<dyn StdError + Send + Sync>;

/// Error object related to Image Handling.
///
/// This is the highest level Error object that the caller would get with underlying `cause` set to
/// the subsystem that caused this error.
#[derive(Debug)]
pub struct ImageError {
    /// Underlying Cause for the Image Error
    cause: Option<Cause>,
}

impl ImageError {
    pub(crate) fn new() -> Self {
        ImageError { cause: None }
    }

    pub(crate) fn with<C: Into<Cause>>(mut self, cause: C) -> Self {
        self.cause = Some(cause.into());
        self
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "ImageError: ({})", cause)
        } else {
            f.write_str("ImageError: (Cause Unknonwn)")
        }
    }
}

impl StdError for ImageError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.cause
            .as_ref()
            .map(|cause| &**cause as &(dyn StdError + 'static))
    }
}

impl From<serde_json::Error> for ImageError {
    fn from(e: serde_json::Error) -> Self {
        ImageError::new().with(e)
    }
}

impl From<ImageError> for std::io::Error {
    fn from(e: ImageError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("{}", e))
    }
}

impl From<std::io::Error> for ImageError {
    fn from(e: std::io::Error) -> Self {
        ImageError::new().with(e)
    }
}
