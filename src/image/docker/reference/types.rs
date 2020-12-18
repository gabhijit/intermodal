use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;

type DockerReferenceResult = Result<DockerReference, DockerReferenceError>;
type DockerRepoResult = Result<DockerRepo, DockerReferenceError>;

pub struct DockerReference {
    repo: DockerRepo,
    tag: String,
    digest: Digest,
}

pub struct DockerRepo {
    domain: String, // Domain where the Repo is hosted.
    path: String,   // Path within the Repo sans Tag
}
