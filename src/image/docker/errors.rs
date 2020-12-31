//! Our Error and Result Types
//!
//!
use std::error::Error as StdError;
use std::fmt;
use std::result::Result;

use super::client::ClientError;
use super::reference::ReferenceError;

pub type DockerImageResult<T> = Result<T, DockerImageError>;

#[derive(Debug)]
pub enum DockerImageError {
    /// Error inside Docker Client
    ClientError(String),

    /// Error inside Docker Reference
    ReferenceError(String),

    /// Error inside Docker Transport
    TransportError(String),

    /// Error inside Docker Image Source
    SourceError(String),

    /// Catchall Error
    GenericError(String),
}

impl StdError for DockerImageError {}

impl fmt::Display for DockerImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DockerImageError::ClientError(ref msg) => write!(f, "Docker Client Error ({})", msg),
            DockerImageError::ReferenceError(ref msg) => {
                write!(f, "Docker Reference Error ({})", msg)
            }
            DockerImageError::TransportError(ref msg) => {
                write!(f, "Docker Transport Error ({})", msg)
            }
            DockerImageError::SourceError(ref msg) => write!(f, "Docker Source Error ({})", msg),
            DockerImageError::GenericError(ref msg) => write!(f, "Docker Generic Error ({})", msg),
        }
    }
}

impl From<ClientError> for DockerImageError {
    fn from(e: ClientError) -> Self {
        DockerImageError::ClientError(format!("{}", e))
    }
}

impl From<ReferenceError> for DockerImageError {
    fn from(e: ReferenceError) -> Self {
        DockerImageError::ReferenceError(format!("{}", e))
    }
}
