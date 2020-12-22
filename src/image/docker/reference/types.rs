use crate::image::docker::transport::DockerTransport;
use crate::image::types::{ImageReference, ImageTransport};
use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;

pub(crate) type DockerReferenceResult<'a> = Result<DockerReference<'a>, DockerReferenceError>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DockerReference<'a> {
    pub(crate) transport: &'a DockerTransport<'a>,
    pub(crate) repo: DockerRepo,
    pub(crate) tag: String,
    pub(crate) digest: Option<Digest>,
    pub(crate) input_ref: String, // The string that was originally sent to us
}

impl<'a> ImageReference for DockerReference<'a> {
    fn transport(&self) -> Box<&(dyn ImageTransport + '_)> {
        Box::new(self.transport)
    }

    fn string_within_transport(&self) -> String {
        let mut s = format!("//{}/{}", self.repo.domain, self.repo.path);
        if !self.tag.is_empty() {
            s.push(':');
            s.push_str(&self.tag);
        }
        if let Some(d) = &self.digest {
            s.push('@');
            s.push_str(&format!("{}", d));
        }
        s
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DockerRepo {
    pub(crate) domain: String, // Domain where the Repo is hosted.
    pub(crate) path: String,   // Path within the Repo sans Tag
}