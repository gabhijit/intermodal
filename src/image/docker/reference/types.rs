#![allow(dead_code)]

//! Types implementing Docker Reference

use std::collections::HashMap;

use crate::image::{
    docker::{
        client::DockerClient, image::DockerImage, source::DockerSource, transport::DockerTransport,
    },
    oci::digest::Digest,
    types::{Image, ImageReference, ImageResult, ImageSource, ImageTransport},
};

use super::errors::ReferenceError;

/// Functions exposed by the 'DockerReference' structure for client.
pub trait DockerImageReference {
    fn name(&self) -> String;

    fn tag(&self) -> String;

    fn digest(&self) -> Option<Digest>;
}

pub(crate) type DockerReferenceResult = Result<DockerReference, ReferenceError>;

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

    pub(crate) fn get_string_within_transport(&self) -> String {
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

impl ImageReference for DockerReference {
    fn transport(&self) -> Box<dyn ImageTransport + Send + Sync> {
        Box::new(DockerTransport::new())
    }

    /// Returns the part of the reference string following the ':'. The returned value is a fully
    /// resolved reference, implying - default paths (like adding '/library') and tags (like
    /// 'latest') and domains are resolved. This is not exactly identical to the passed input
    /// reference. However, a reference if obtained from this string, is equivalent to the
    /// reference that would be returned for the user supplied input.
    fn string_within_transport(&self) -> String {
        self.get_string_within_transport()
    }

    /// Returns an object implementing trait 'ImageSource' (in our case 'DockerSource').
    fn new_image_source(&self) -> ImageResult<Box<dyn ImageSource + Send + Sync>> {
        let domain = self.domain();
        let client = DockerClient::new(domain);

        Ok(Box::new(DockerSource {
            reference: self.clone(),
            client,
            manifest_cache: HashMap::new(),
        }))
    }

    /// Returns an object implementing trait 'Image' in our case 'DockerImage'
    fn new_image(&self) -> ImageResult<Box<dyn Image>> {
        let source = self.new_image_source()?;

        // FIXME: Get a proper manifest.
        let manifest: Vec<u8> = vec![];

        Ok(Box::new(DockerImage {
            source,
            manifest,
            cfgblob: None,
        }))
    }

    fn docker_reference(&self) -> Option<Box<dyn DockerImageReference>> {
        Some(Box::new(self.clone()))
    }
}

impl DockerImageReference for DockerReference {
    fn name(&self) -> String {
        format!("{}/{}", self.domain(), self.path())
    }

    fn tag(&self) -> String {
        self.tag.clone()
    }

    fn digest(&self) -> Option<Digest> {
        self.digest.clone()
    }
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DockerRepo {
    pub(crate) domain: String, // Domain where the Repo is hosted.
    pub(crate) path: String,   // Path within the Repo sans Tag
}

#[cfg(test)]
mod tests {

    use crate::image::docker::reference::api::parse;

    #[test]
    fn test_default_domain_for_image() {
        let image_ref = parse("fedora").unwrap();

        assert_eq!(image_ref.domain(), "docker.io");
        assert_eq!(image_ref.path(), "library/fedora");
    }
}
