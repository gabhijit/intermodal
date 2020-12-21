use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DockerReferenceError {
    InvalidFormatError,
    InvalidTagError,
    InvalidDigestError,
    EmptyNameError,
    NameTooLongError,
    NameNotCanonicalError,

    // Unknown Error
    UnknownDockerReferenceError,
}

impl fmt::Display for DockerReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DockerReferenceError::InvalidFormatError => write!(f, "Invalid Reference Format!"),
            DockerReferenceError::InvalidTagError => write!(f, "Invalid Reference Tag!"),
            DockerReferenceError::InvalidDigestError => write!(f, "Invalid Reference Digest!"),
            DockerReferenceError::EmptyNameError => write!(f, "Empty Reference Name!"),
            DockerReferenceError::NameTooLongError => write!(f, "Reference Name Too Long!"),
            DockerReferenceError::NameNotCanonicalError => {
                write!(f, "Reference Name Not Canonical!")
            }
            DockerReferenceError::UnknownDockerReferenceError => {
                write!(f, "Unknown Error occurred!")
            }
        }
    }
}
impl Error for DockerReferenceError {}
