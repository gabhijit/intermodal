#![allow(non_upper_case_globals)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::oci::digest::Digest;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Descriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mediatype: Option<String>,

    digest: Digest,

    size: i64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    urls: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Platform {
    architecture: String,

    os: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    os_version: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    os_features: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    variant: Option<String>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Index {
    #[serde(rename = "schemaVersion")]
    version: u8,

    manifests: Vec<Descriptor>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    annotations: Option<HashMap<String, String>>,
}

// FIXME: Not sure what to do with the constants?

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ImageLayout {
    #[serde(rename = "imageLayoutVersion")]
    img_layout_version: String,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    version: u8,

    config: Descriptor,

    layers: Vec<Descriptor>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    annotations: Option<HashMap<String, String>>,
}

pub const MediaTypeDescriptor: &str = "application/vnd.oci.descriptor.v1+json";

pub const MediaTypeLayoutHeader: &str = "application/vnd.oci.layout.header.v1+json";

pub const MediaTypeImageManifest: &str = "application/vnd.oci.image.manifest.v1+json";

pub const MediaTypeImageIndex: &str = "application/vnd.oci.image.index.v1+json";

pub const MediaTypeImageLayer: &str = "application/vnd.oci.image.layer.v1.tar";

pub const MediaTypeImageLayerGzip: &str = "application/vnd.oci.image.layer.v1.tar+gzip";

pub const MediaTypeImageLayerZstd: &str = "application/vnd.oci.image.layer.v1.tar+zstd";

pub const MediaTypeImageLayerNonDistributable: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar";

pub const MediaTypeImageLayerNonDistributableGzip: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip";

pub const MediaTypeImageLayerNonDistributableZstd: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd";

pub const MediaTypeImageConfig: &str = "application/vnd.oci.image.config.v1+json";

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ImageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "User")]
    user: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ExposedPorts"
    )]
    exposed_ports: Option<HashMap<String, String>>, // FIXME: Use correct type

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Env")]
    env: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "EntryPoint"
    )]
    entry_point: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Cmd")]
    cmd: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Volumes")]
    volumes: Option<HashMap<String, String>>, // FIXME: Use correct type

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "WorkingDir"
    )]
    working_dir: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Labels")]
    labels: Option<HashMap<String, String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "StopSignal"
    )]
    stop_signal: Option<String>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RootFS {
    type_: String,

    diff_ids: Vec<String>, // FIXME: This should be proper digest type.
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct History {
    created: DateTime<Utc>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    created_by: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    author: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    comment: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    empty_layer: Option<bool>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Image {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    created: Option<DateTime<Utc>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    author: Option<String>,

    architecture: String,

    os: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    config: Option<ImageConfig>,

    rootfs: RootFS,

    history: Option<History>,
}
