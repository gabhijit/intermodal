//! Implementation of Docker specific ImageSource

use async_trait::async_trait;

use crate::image::types::errors::ImageResult;
use crate::image::types::{ImageManifest, ImageReference, ImageSource};
use crate::oci::digest::Digest;

use super::client::DockerClient;
use super::reference::types::DockerReference;

#[derive(Clone, Debug)]
pub(crate) struct DockerSource {
    pub(crate) reference: DockerReference,
    pub(super) client: DockerClient,
}

impl DockerSource {
    async fn cached_or_fetch_manifest(
        &mut self,
        digest: Option<&Digest>,
    ) -> ImageResult<ImageManifest> {
        let digest_or_tag = if digest.is_none() {
            if self.reference.digest.is_none() {
                self.reference.tag.clone()
            } else {
                self.reference.digest.as_ref().unwrap().to_string()
            }
        } else {
            digest.unwrap().to_string()
        };

        /* let digest_or_tag = if self.reference.digest.is_none() {
            &self.reference.tag
        } else {
            &digest_str
        };*/

        Ok(self
            .client
            .do_get_manifest(self.reference.path(), &digest_or_tag)
            .await?)
    }
}

#[async_trait]
impl ImageSource for DockerSource {
    fn reference(&self) -> Box<dyn ImageReference> {
        Box::new(self.reference.clone())
    }

    async fn get_manifest(&mut self, digest: Option<&Digest>) -> ImageResult<ImageManifest> {
        Ok(self.cached_or_fetch_manifest(digest).await?)
    }
}
