use std::fmt::Display;

use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;

pub type DockerReferenceResult = Result<DockerReference, DockerReferenceError>;
pub type DockerRepoResult = Result<DockerRepo, DockerReferenceError>;

#[derive(Debug, PartialEq, Eq)]
pub struct DockerReference {
    pub(super) repo: DockerRepo,
    pub(super) tag: String,
    pub(super) digest: Option<Digest>,
    pub(super) input_ref: String, // The string that was originally sent to us
}

#[derive(Debug, PartialEq, Eq)]
pub struct DockerRepo {
    pub(super) domain: String, // Domain where the Repo is hosted.
    pub(super) path: String,   // Path within the Repo sans Tag
}
