//! Types implementing Docker Reference

use crate::image::docker::{
    client::DockerClient, source::DockerSource, transport::DockerTransport,
};
use crate::image::types::{ImageReference, ImageResult, ImageSource, ImageTransport};
use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;

pub(crate) type DockerReferenceResult = Result<DockerReference, DockerReferenceError>;

/// A structure implementing Docker Reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DockerReference {
    pub(crate) repo: DockerRepo,
    pub(crate) tag: String,
    pub(crate) digest: Option<Digest>,
    pub(crate) input_ref: String, // The string that was originally sent to us
}

impl DockerReference {
    pub fn domain(&self) -> &str {
        &self.repo.domain
    }

    pub fn path(&self) -> &str {
        &self.repo.path
    }
}

impl ImageReference for DockerReference {
    fn transport(&self) -> Box<dyn ImageTransport + Send + Sync> {
        Box::new(DockerTransport::new())
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

    fn new_image_source(&self) -> ImageResult<Box<dyn ImageSource>> {
        let domain = self.domain();
        let client = DockerClient::new(domain);

        Ok(Box::new(DockerSource {
            reference: self.clone(),
            client,
        }))
    }
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DockerRepo {
    pub(crate) domain: String, // Domain where the Repo is hosted.
    pub(crate) path: String,   // Path within the Repo sans Tag
}
