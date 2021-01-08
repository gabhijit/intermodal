//! Implementation of Docker specific ImageSource
use std::collections::HashMap;

use async_trait::async_trait;

use crate::image::types::errors::ImageResult;
use crate::image::types::{ImageManifest, ImageReference, ImageSource};
use crate::oci::digest::Digest;

use super::client::DockerClient;
use super::reference::types::DockerReference;

/// DockerSource structure. This structure implements `ImageSource` trait.
#[derive(Clone, Debug)]
pub(crate) struct DockerSource {
    pub(crate) reference: DockerReference,
    pub(super) client: DockerClient,
    pub(crate) manifest_cache: HashMap<String, ImageManifest>,
}

impl DockerSource {
    async fn cached_or_fetch_manifest(
        &mut self,
        digest: Option<&Digest>,
    ) -> ImageResult<ImageManifest> {
        let digest_or_tag = if digest.is_none() {
            if self.reference.digest.is_none() {
                log::trace!(
                    "Empty Reference Digest. Using the Tag (default or specified) to get the manifest!"
                );
                self.reference.tag.clone()
            } else {
                let s = self.reference.digest.as_ref().unwrap().to_string();
                log::trace!("Reference Digest Found {}", &s);
                s
            }
        } else {
            digest.unwrap().to_string()
        };

        if self.manifest_cache.contains_key(&digest_or_tag) {
            log::trace!("Cached Manifest found: Returning Cached!");
            return Ok(self.manifest_cache.get(&digest_or_tag).unwrap().clone());
        }

        log::trace!("Downloading Manifest!");
        let manifest = self
            .client
            .do_get_manifest(self.reference.path(), &digest_or_tag)
            .await?;

        log::trace!("Saving Manifest in the cache!");
        self.manifest_cache.insert(digest_or_tag, manifest.clone());

        Ok(manifest)
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

    async fn get_blob(&mut self, digest: &Digest) -> ImageResult<Vec<u8>> {
        Ok(self
            .client
            .do_get_blob(self.reference.path(), digest)
            .await?)
    }
}
