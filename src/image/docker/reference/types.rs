use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;

pub type DockerReferenceResult = Result<DockerReference, DockerReferenceError>;
pub type DockerRepoResult = Result<DockerRepo, DockerReferenceError>;

pub struct DockerReference {
    pub(super) repo: DockerRepo,
    pub(super) tag: String,
    pub(super) digest: Option<Digest>,
}

pub struct DockerRepo {
    pub(super) domain: String, // Domain where the Repo is hosted.
    pub(super) path: String,   // Path within the Repo sans Tag
}
