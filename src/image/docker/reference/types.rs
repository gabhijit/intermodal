use crate::image::docker::transport::DockerTransport;
use crate::image::types::{ImageReference, ImageTransport};
use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;

pub(crate) type DockerReferenceResult<'a> = Result<DockerReference<'a>, DockerReferenceError>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DockerReference<'a> {
    pub(super) transport: &'a DockerTransport<'a>,
    pub(super) repo: DockerRepo,
    pub(super) tag: String,
    pub(super) digest: Option<Digest>,
    pub(super) input_ref: String, // The string that was originally sent to us
}

impl<'a> ImageReference for DockerReference<'a> {
    fn transport(&self) -> Box<&(dyn ImageTransport + '_)> {
        Box::new(self.transport)
    }

    fn string_within_transport(&self) -> String {
        let mut s = String::from(format!("{}/{}", self.repo.domain, self.repo.path));
        if !self.tag.is_empty() {
            s.push_str(&self.tag);
        } else {
            if let Some(d) = &self.digest {
                s.push_str(&format!("{}", d));
            }
        }
        s
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DockerRepo {
    pub(super) domain: String, // Domain where the Repo is hosted.
    pub(super) path: String,   // Path within the Repo sans Tag
}
