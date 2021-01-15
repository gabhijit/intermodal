use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::image::oci::digest::Digest;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Descriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mediatype: Option<String>,

    pub digest: Digest,

    pub size: i64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Platform {
    pub architecture: String,

    pub os: String,

    #[serde(
        default,
        rename = "os.version",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_version: Option<String>,

    #[serde(
        default,
        rename = "os.features",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_features: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Index {
    #[serde(rename = "schemaVersion")]
    pub version: u8,

    pub manifests: Vec<Descriptor>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
}

// FIXME: Not sure what to do with the constants?

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ImageLayout {
    #[serde(rename = "imageLayoutVersion")]
    pub img_layout_version: String,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    pub version: u8,

    pub config: Descriptor,

    pub layers: Vec<Descriptor>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
}

pub const MEDIA_TYPE_DESCRIPTOR: &str = "application/vnd.oci.descriptor.v1+json";

pub const MEDIA_TYPE_LAYOUT_HEADER: &str = "application/vnd.oci.layout.header.v1+json";

pub const MEDIA_TYPE_IMAGE_MANIFEST: &str = "application/vnd.oci.image.manifest.v1+json";

pub const MEDIA_TYPE_IMAGE_INDEX: &str = "application/vnd.oci.image.index.v1+json";

pub const MEDIA_TYPE_IMAGE_LAYER: &str = "application/vnd.oci.image.layer.v1.tar";

pub const MEDIA_TYPE_IMAGE_LAYER_GZIP: &str = "application/vnd.oci.image.layer.v1.tar+gzip";

pub const MEDIA_TYPE_IMAGE_LAYER_ZSTD: &str = "application/vnd.oci.image.layer.v1.tar+zstd";

pub const MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar";

pub const MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE_GZIP: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip";

pub const MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE_ZSTD: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd";

pub const MEDIA_TYPE_IMAGE_CONFIG: &str = "application/vnd.oci.image.config.v1+json";

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ImageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "User")]
    pub user: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ExposedPorts"
    )]
    pub exposed_ports: Option<HashMap<String, String>>, // FIXME: Use correct type

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Env")]
    pub env: Option<Vec<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "EntryPoint"
    )]
    pub entry_point: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Cmd")]
    pub cmd: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Volumes")]
    pub volumes: Option<HashMap<String, String>>, // FIXME: Use correct type

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "WorkingDir"
    )]
    pub working_dir: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "StopSignal"
    )]
    pub stop_signal: Option<String>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RootFS {
    #[serde(default, rename = "type")]
    pub type_: String,

    pub diff_ids: Vec<String>, // FIXME: This should be proper digest type.
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct History {
    pub created: DateTime<Utc>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_layer: Option<bool>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Image {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    pub architecture: String,

    pub os: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<ImageConfig>,

    pub rootfs: RootFS,

    pub history: Option<Vec<History>>,
}
