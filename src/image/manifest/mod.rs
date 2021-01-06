//! Image manifest handling related

use lazy_static::lazy_static;

use crate::image::docker::{MEDIA_TYPE_DOCKER_V2_LIST, MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST};
use crate::image::types::{
    errors::{ImageError, ImageResult},
    BlobInfo, ImageManifest, ImageSource,
};
use crate::oci::image::spec_v1::{
    Image as OCISpecv1Image, MEDIA_TYPE_IMAGE_INDEX, MEDIA_TYPE_IMAGE_MANIFEST,
};

lazy_static! {
    pub(crate) static ref DEFAULT_SUPPORTED_MANIFESTS: Vec<&'static str> = vec![
        MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST,
        MEDIA_TYPE_DOCKER_V2_LIST,
        MEDIA_TYPE_IMAGE_INDEX,
        MEDIA_TYPE_IMAGE_MANIFEST,
    ];
}

/// A Generic Manifest Trait
///
/// Reference:: github.com/containers/image/image/manifest.go genericManifest interface
pub(super) trait GenericManifest {
    /// Serialize the Manifest to the Blob
    fn serialize(&self) -> ImageResult<Vec<u8>>;

    fn mime_type(&self) -> String;

    fn config_info(&self) -> BlobInfo; // FIXME : Add this

    fn config_blog(&self) -> ImageResult<Vec<u8>>;

    fn oci_config(&self) -> ImageResult<OCISpecv1Image>;

    fn layer_infos(&self) -> Vec<BlobInfo>;
}

pub(super) fn manifest_instance_from_blob(
    src: &Box<dyn ImageSource>,
    manifest: &ImageManifest,
) -> ImageResult<Box<dyn GenericManifest>> {
    Err(ImageError::new())
}
