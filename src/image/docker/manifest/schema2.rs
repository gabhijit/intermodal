//! Structs Describing Docker Schema2 Manifest etc.
//!
//! Note: We are supporting only the 'list' and 'schema2' descriptors from docker/distribution

use std::collections::HashMap;
use std::time::Duration as StdDuration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::oci::digest::Digest;

/// A Descriptor in docker/distribution Schema 2
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2Descriptor {
    #[serde(rename = "mediaType")]
    pub media_type: String,

    pub size: i64,

    pub digest: Digest,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// A Manifest in a docker/distribution Schema 2
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i8,

    #[serde(rename = "mediaType")]
    pub media_type: String,

    pub config: Schema2Descriptor,

    pub layers: Vec<Schema2Descriptor>,
}

/// Configuration Settings for the HEALTHCHECKER features.
///
/// From docker/api/types/container
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2HealthConfig {
    #[serde(rename = "Test", skip_serializing_if = "Option::is_none")]
    pub test: Option<Vec<String>>,

    #[serde(rename = "StartPeriod", skip_serializing_if = "Option::is_none")]
    pub start_period: Option<StdDuration>,

    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<StdDuration>,

    #[serde(rename = "Timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<StdDuration>,

    #[serde(rename = "Retries", skip_serializing_if = "Option::is_none")]
    pub retrie: Option<i16>,
}

/// Schema2Config from docker/api/types/container
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2Config {
    #[serde(rename = "Hostname")]
    pub hostname: String,

    #[serde(rename = "Domainname")]
    pub domainname: String,

    #[serde(rename = "User")]
    pub user: String,

    #[serde(rename = "AttachStdin")]
    pub attach_stdin: bool,

    #[serde(rename = "AttachStdout")]
    pub attach_stdout: bool,

    #[serde(rename = "AttachStderr")]
    pub attach_stderr: bool,

    #[serde(rename = "ExposedPorts", skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<String>, // FIXME:

    #[serde(rename = "Tty")]
    pub tty: bool,

    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,

    #[serde(rename = "StdinOnce")]
    pub stdin_once: bool,

    #[serde(rename = "Env")]
    pub env: Vec<String>,

    #[serde(rename = "Cmd")]
    pub cmd: String,

    #[serde(rename = "HealthCheck", skip_serializing_if = "Option::is_none")]
    pub health_check: Option<Schema2HealthConfig>,

    #[serde(rename = "ArgsEscaped", skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "Volumes")]
    pub volumes: Vec<String>,

    #[serde(rename = "WorkingDir")]
    pub working_dir: String,

    #[serde(rename = "EntryPoint")]
    pub entry_point: String,

    #[serde(rename = "NetworkDisabled", skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,

    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,

    #[serde(rename = "OnBuild")]
    pub on_build: Vec<String>,

    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "StopSignal", skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,

    #[serde(rename = "StopTimeout", skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i16>,

    #[serde(rename = "Shell", skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>, // FIXME: &'static str?
}

// FIXME: Get this right
/// Schema2Image struct from docker/docker/image
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    pub created: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_config: Option<Schema2Config>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Schema2Config>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rootfs: Option<Schema2RootFS>,

    #[serde(skip_serializing_if = "Option::is_none")]
    history: Option<Schema2History>,

    #[serde(rename = "os.version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,

    #[serde(rename = "os.features", skip_serializing_if = "Option::is_none")]
    pub os_features: Option<String>,
}

/// RootFS Struct
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2RootFS {
    #[serde(rename = "type")]
    pub type_: String,

    pub diff_ids: Vec<Digest>,
}

/// Schema2History Struct
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2History {
    pub created: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_layer: Option<bool>,
}

/// Schema2PlatformSpec Struct
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2PlatformSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    pub os: String,

    #[serde(rename = "os.version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,

    #[serde(rename = "os.features", skip_serializing_if = "Option::is_none")]
    pub os_features: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
}

/// Schema2Manifest Descriptor
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2ManifestDescriptor {
    #[serde(rename = "mediaType")]
    pub media_type: String,

    pub size: i64,

    pub digest: Digest,

    pub platform: Schema2PlatformSpec,
}

/// Schema2List Structure
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2List {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i8,

    #[serde(rename = "mediaType")]
    pub media_type: String,

    pub manifests: Vec<Schema2ManifestDescriptor>,
}
