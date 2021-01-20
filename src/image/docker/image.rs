//! Implementation of a 'trait Image' for Docker

use async_trait::async_trait;
use bytes::BufMut;
use futures_util::StreamExt;

use crate::image::{
    docker::{MEDIA_TYPE_DOCKER_V2_LIST, MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST},
    oci::spec_v1::Image as OCIv1Image,
    platform::get_os_platform,
    types::{
        errors::{ImageError, ImageResult},
        Image, ImageInspect, ImageManifest, ImageReference, ImageSource,
    },
};

use super::manifest::schema2::{Schema2, Schema2Config, Schema2Image, Schema2List};

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
    pub cfgblob: Option<Vec<u8>>,
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

    fn source_ref(&self) -> &dyn ImageSource {
        self.source.as_ref()
    }

    async fn manifest(&mut self) -> ImageResult<ImageManifest> {
        Ok(self.source.get_manifest(None).await?)
    }

    async fn resolved_manifest(&mut self) -> ImageResult<ImageManifest> {
        let original = self.source.get_manifest(None).await?;

        Ok(self.resolve_manifest(&original).await?)
    }

    async fn config_blob(&mut self) -> ImageResult<Vec<u8>> {
        if self.cfgblob.is_none() {
            log::debug!("Config blob is not cached. Downloading Config blob.");
            let manifest = self.resolved_manifest().await?;
            let schema: Schema2 = serde_json::from_slice(&manifest.manifest)?;
            let cfgblob = self.source.get_blob(&schema.config.digest).await?;

            futures_util::pin_mut!(cfgblob);

            let mut blobvec = Vec::new();

            while let Some(data) = cfgblob.next().await {
                blobvec.put(data);
            }

            self.cfgblob = Some(blobvec);
            log::trace!(
                "Config Blob: {}",
                String::from_utf8(self.cfgblob.as_ref().unwrap().to_vec()).unwrap()
            );
        }
        Ok(self.cfgblob.as_ref().unwrap().clone())
    }

    async fn oci_config(&mut self) -> ImageResult<OCIv1Image> {
        Ok(serde_json::from_slice(&self.config_blob().await?)?)
    }

    async fn inspect(&mut self) -> ImageResult<ImageInspect> {
        let manifest: Schema2 = serde_json::from_slice(&self.resolved_manifest().await?.manifest)?;
        let layers: Vec<String> = manifest
            .layers
            .iter()
            .map(|l| l.digest.to_string())
            .collect();

        log::debug!("{}", String::from_utf8(self.config_blob().await?).unwrap());

        let docker_image: Schema2Image = serde_json::from_slice(&self.config_blob().await?)?;
        let docker_config = docker_image.config.as_ref();

        Ok(ImageInspect {
            created: docker_image.created.to_string(),
            architecture: docker_image.architecture.unwrap_or_default(),
            docker_version: docker_image.docker_version.unwrap_or_default(),
            os: docker_image.os.unwrap_or_default(),
            layers,
            labels: docker_config
                .unwrap_or(&Schema2Config::default())
                .labels
                .clone(),

            env: docker_config
                .unwrap_or(&Schema2Config::default())
                .env
                .clone(),
        })
    }
}
