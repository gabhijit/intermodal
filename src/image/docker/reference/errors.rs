use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum ReferenceError {
    InvalidFormat,
    InvalidTag,
    InvalidDigest,
    EmptyName,
    NameTooLong,
    NameNotCanonical,

    // Unknown Error
    Unknown,
}

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ReferenceError::InvalidFormat => write!(f, "Invalid Reference Format!"),
            ReferenceError::InvalidTag => write!(f, "Invalid Reference Tag!"),
            ReferenceError::InvalidDigest => write!(f, "Invalid Reference Digest!"),
            ReferenceError::EmptyName => write!(f, "Empty Reference Name!"),
            ReferenceError::NameTooLong => write!(f, "Reference Name Too Long!"),
            ReferenceError::NameNotCanonical => {
                write!(f, "Reference Name Not Canonical!")
            }
            ReferenceError::Unknown => {
                write!(f, "Unknown Error occurred!")
            }
        }
    }
}
impl Error for ReferenceError {}
