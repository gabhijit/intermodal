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
    hostname: String,

    #[serde(rename = "Domainname")]
    domainname: String,

    #[serde(rename = "User")]
    user: String,

    #[serde(rename = "AttachStdin")]
    attach_stdin: bool,

    #[serde(rename = "AttachStdout")]
    attach_stdout: bool,

    #[serde(rename = "AttachStderr")]
    attach_stderr: bool,

    #[serde(rename = "ExposedPorts", skip_serializing_if = "Option::is_none")]
    exposed_ports: Option<String>, // FIXME:

    #[serde(rename = "Tty")]
    tty: bool,

    #[serde(rename = "OpenStdin")]
    open_stdin: bool,

    #[serde(rename = "StdinOnce")]
    stdin_once: bool,

    #[serde(rename = "Env")]
    env: Vec<String>,

    #[serde(rename = "Cmd")]
    cmd: String,

    #[serde(rename = "HealthCheck", skip_serializing_if = "Option::is_none")]
    health_check: Option<Schema2HealthConfig>,

    #[serde(rename = "ArgsEscaped", skip_serializing_if = "Option::is_none")]
    args_escaped: Option<bool>,

    #[serde(rename = "Image")]
    image: String,

    #[serde(rename = "Volumes")]
    volumes: Vec<String>,

    #[serde(rename = "WorkingDir")]
    working_dir: String,

    #[serde(rename = "EntryPoint")]
    entry_point: String,

    #[serde(rename = "NetworkDisabled", skip_serializing_if = "Option::is_none")]
    network_disabled: Option<bool>,

    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    mac_address: Option<String>,

    #[serde(rename = "OnBuild")]
    on_build: Vec<String>,

    #[serde(rename = "Labels")]
    labels: HashMap<String, String>,

    #[serde(rename = "StopSignal", skip_serializing_if = "Option::is_none")]
    stop_signal: Option<String>,

    #[serde(rename = "StopTimeout", skip_serializing_if = "Option::is_none")]
    stop_timeout: Option<i16>,

    #[serde(rename = "Shell", skip_serializing_if = "Option::is_none")]
    shell: Option<String>, // FIXME: &'static str?
}

// FIXME: Get this right
/// Schema2Image struct from docker/docker/image
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    created: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_config: Option<Schema2Config>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Schema2Config>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
}

/// RootFS Struct
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2RootFS {
    #[serde(rename = "type")]
    type_: String,

    diff_ids: Vec<Digest>,
}

/// Schema2History Struct
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema2History {
    created: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    empty_layer: Option<bool>,
}
