//! Implementation of Schema2 Manifest Functionality
//!
//! This module implements functionality needed to deal with Schema2 Manifests.

use crate::image::types::ImageSource;

use super::schema2;

#[derive(Debug)]
pub(crate) struct DockerManifestSchema2 {
    _source: Box<dyn ImageSource>,
    _schema: schema2::Schema2,
}
