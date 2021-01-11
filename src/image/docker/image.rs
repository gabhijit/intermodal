//! Implementation of a 'trait Image' for Docker

use async_trait::async_trait;

use crate::image::docker::{MEDIA_TYPE_DOCKER_V2_LIST, MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST};
use crate::image::platform::get_os_platform;
use crate::image::types::{
    errors::ImageError, Image, ImageManifest, ImageReference, ImageResult, ImageSource,
};
use crate::oci::image::spec_v1::Image as OCIv1Image;

use super::manifest::schema2::{Schema2, Schema2List};

/// A `DockerImage` is a resolved Image which contains a source (`DockerSource`) and a 'blob' that
/// can be deserialized to  a `Schema2` struct.
///
/// Note: The 'resolved' manifest will be a manifest that points to an 'instance' of an image and
/// not the 'manifest' retured by the `get_manifest` on the source above, which could return an
/// instance of a `list` or `index`. The 'resolved' manifest will be the one that is specific to
/// current OS/Arch
#[derive(Debug)]
pub struct DockerImage {
    pub source: Box<dyn ImageSource + Send + Sync>,
    pub manifest: Vec<u8>,
}

impl DockerImage {
    async fn manifest_for_our_os_arch(
        &mut self,
        original: &ImageManifest,
    ) -> ImageResult<ImageManifest> {
        let mime_type = original.mime_type.as_str();

        log::debug!("Getting the Manifest for Current OS/Architecture");
        match mime_type {
            MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST => {
                log::trace!("Current Manifest is not a List, So using it as it is!");
                Ok(original.clone())
            }
            MEDIA_TYPE_DOCKER_V2_LIST => {
                log::trace!(
                    "Found Manifest List, Getting the actual manifest matching, OS/Platform"
                );
                let list: Schema2List = serde_json::from_slice(&original.manifest)?;
                for m in list.manifests.iter() {
                    let (architecture, os) =
                        (m.platform.architecture.as_ref(), m.platform.os.clone());
                    let platform = get_os_platform();
                    if &platform.architecture == architecture.unwrap() && platform.os == os {
                        log::trace!("Getting Manifest for Digest: {}", m.digest);
                        return Ok(self.source.get_manifest(Some(&m.digest)).await?);
                    }
                }
                log::error!("No Manifest found Matching Current OS/Platform!");
                // FIXME: Get a proper Error type
                Err(ImageError::new())
            }
            _ => {
                log::error!(
                    "Media Type: {} found. Can't Download Manifest for this Media Type.",
                    mime_type
                );
                Err(ImageError::new())
            }
        }
    }

    async fn resolve_manifest(&mut self, original: &ImageManifest) -> ImageResult<ImageManifest> {
        Ok(self.manifest_for_our_os_arch(original).await?)
    }
}

#[async_trait]
impl Image for DockerImage {
    fn reference(&self) -> Box<dyn ImageReference> {
        self.source.reference()
    }

    async fn manifest(&mut self) -> ImageResult<ImageManifest> {
        let original = self.source.get_manifest(None).await?;

        Ok(self.resolve_manifest(&original).await?)
    }

    async fn config_blob(&mut self) -> ImageResult<Vec<u8>> {
        let manifest = self.manifest().await?;
        let schema: Schema2 = serde_json::from_slice(&manifest.manifest)?;
        Ok(self.source.get_blob(&schema.config.digest).await?)
    }

    async fn oci_config(&mut self) -> ImageResult<OCIv1Image> {
        Ok(serde_json::from_slice(&self.config_blob().await?)?)
    }
}
