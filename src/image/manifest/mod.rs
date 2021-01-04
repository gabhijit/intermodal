//! Image manifest handling related

use lazy_static::lazy_static;

use crate::image::docker::{MEDIA_TYPE_DOCKER_V2_LIST, MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST};
use crate::oci::image::spec_v1::{MEDIA_TYPE_IMAGE_INDEX, MEDIA_TYPE_IMAGE_MANIFEST};

lazy_static! {
    pub(crate) static ref DEFAULT_SUPPORTED_MANIFESTS: Vec<&'static str> = vec![
        MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST,
        MEDIA_TYPE_DOCKER_V2_LIST,
        MEDIA_TYPE_IMAGE_INDEX,
        MEDIA_TYPE_IMAGE_MANIFEST,
    ];
}
