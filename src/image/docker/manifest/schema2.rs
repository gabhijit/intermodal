//! Structs Describing Docker Schema2 Manifest etc.
//!
//! Note: We are supporting only the 'list' and 'schema2' descriptors from docker/distribution

use std::collections::HashMap;
use std::time::Duration as StdDuration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::image::oci::digest::Digest;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Empty {}

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
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    pub exposed_ports: Option<HashMap<String, Empty>>,

    #[serde(rename = "Tty")]
    pub tty: bool,

    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,

    #[serde(rename = "StdinOnce")]
    pub stdin_once: bool,

    #[serde(default, rename = "Env", deserialize_with = "deserialize_null_default")]
    pub env: Vec<String>,

    #[serde(default, rename = "Cmd", deserialize_with = "deserialize_null_default")]
    pub cmd: Vec<String>,

    #[serde(rename = "HealthCheck", skip_serializing_if = "Option::is_none")]
    pub health_check: Option<Schema2HealthConfig>,

    #[serde(rename = "ArgsEscaped", skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,

    #[serde(rename = "WorkingDir")]
    pub working_dir: String,

    #[serde(rename = "EntryPoint", skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,

    #[serde(rename = "NetworkDisabled", skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,

    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,

    #[serde(rename = "OnBuild", skip_serializing_if = "Option::is_none")]
    pub on_build: Option<Vec<String>>,

    #[serde(rename = "Labels", deserialize_with = "deserialize_null_default")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "StopSignal", skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,

    #[serde(rename = "StopTimeout", skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i16>,

    #[serde(rename = "Shell", skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>, // FIXME: &'static str?
}

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
    history: Option<Vec<Schema2History>>,

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

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample_fedoraproject_schema2image() {
        let input = r#"
{"architecture": "amd64", "comment": "Created by Image Factory", "config": {"AttachStderr": false, "AttachStdin": false, "AttachStdout": false, "Cmd": ["/bin/bash"], "Domainname": "", "Entrypoint": null, "Env": ["DISTTAG=f33container", "FGC=f33", "container=oci"], "ExposedPorts": null, "Hostname": "", "Image": "", "Labels": {"name": "fedora", "license": "MIT", "vendor": "Fedora Project", "version": "33"}, "MacAddress": "", "NetworkDisabled": false, "OnBuild": null, "OpenStdin": false, "StdinOnce": false, "Systemd": false, "Tty": false, "User": "", "VolumeDriver": "", "Volumes": null, "WorkingDir": ""}, "container_config": {"AttachStderr": false, "AttachStdin": false, "AttachStdout": false, "Cmd": null, "Domainname": "", "Entrypoint": null, "Env": null, "ExposedPorts": null, "Hostname": "", "Image": "", "Labels": null, "MacAddress": "", "NetworkDisabled": false, "OnBuild": null, "OpenStdin": false, "StdinOnce": false, "Systemd": false, "Tty": false, "User": "", "VolumeDriver": "", "Volumes": null, "WorkingDir": ""}, "created": "2020-10-27T07:49:11Z", "docker_version": "1.10.1", "os": "linux", "history": [{"comment": "Created by Image Factory", "created": "2020-10-27T07:49:11Z"}], "rootfs": {"diff_ids": ["sha256:b4fa6ff1346dec95ce4454464201fdadfd816e10eb7322048829c551ce032d08"], "type": "layers"}}
    "#;
        let parsed = serde_json::from_str::<Schema2Image>(input);
        assert!(parsed.is_ok(), "{}", parsed.err().unwrap())
    }

    #[test]
    fn test_sample_quay_io_schema2image() {
        let input = r#" {"architecture": "amd64", "comment": "Created by Image Factory", "config": {"AttachStderr": false, "AttachStdin": false, "AttachStdout": false, "Cmd": ["/bin/bash"], "Domainname": "", "Entrypoint": null, "Env": ["DISTTAG=f33container", "FGC=f33", "container=oci"], "ExposedPorts": null, "Hostname": "", "Image": "", "Labels": {"name": "fedora", "license": "MIT", "vendor": "Fedora Project", "version": "33"}, "MacAddress": "", "NetworkDisabled": false, "OnBuild": null, "OpenStdin": false, "StdinOnce": false, "Systemd": false, "Tty": false, "User": "", "VolumeDriver": "", "Volumes": null, "WorkingDir": ""}, "container_config": {"AttachStderr": false, "AttachStdin": false, "AttachStdout": false, "Cmd": null, "Domainname": "", "Entrypoint": null, "Env": null, "ExposedPorts": null, "Hostname": "", "Image": "", "Labels": null, "MacAddress": "", "NetworkDisabled": false, "OnBuild": null, "OpenStdin": false, "StdinOnce": false, "Systemd": false, "Tty": false, "User": "", "VolumeDriver": "", "Volumes": null, "WorkingDir": ""}, "created": "2020-10-27T07:49:11Z", "docker_version": "1.10.1", "os": "linux", "history": [{"comment": "Created by Image Factory", "created": "2020-10-27T07:49:11Z"}], "rootfs": {"diff_ids": ["sha256:b4fa6ff1346dec95ce4454464201fdadfd816e10eb7322048829c551ce032d08"], "type": "layers"}}
            "#;
        let parsed = serde_json::from_str::<Schema2Image>(input);
        assert!(parsed.is_ok(), "{}", parsed.err().unwrap())
    }

    #[test]
    fn test_sample_docker_io_schema2image() {
        let input = r##"
        {"architecture":"amd64","config":{"Hostname":"","Domainname":"","User":"","AttachStdin":false,"AttachStdout":false,"AttachStderr":false,"Tty":false,"OpenStdin":false,"StdinOnce":false,"Env":["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin","DISTTAG=f33container","FGC=f33","FBR=f33"],"Cmd":["/bin/bash"],"Image":"sha256:3b1b0c55a47e10ea93d904fc20c39d253f9e1ad770922e8fb4af93dcec6691ce","Volumes":null,"WorkingDir":"","Entrypoint":null,"OnBuild":null,"Labels":{"maintainer":"Clement Verna \u003ccverna@fedoraproject.org\u003e"}},"container":"50cf73b69958473ab2f9a10d3249df073c99b7767ec7f1ff5ffd56da4f35397b","container_config":{"Hostname":"50cf73b69958","Domainname":"","User":"","AttachStdin":false,"AttachStdout":false,"AttachStderr":false,"Tty":false,"OpenStdin":false,"StdinOnce":false,"Env":["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin","DISTTAG=f33container","FGC=f33","FBR=f33"],"Cmd":["/bin/sh","-c","#(nop) ","CMD [\"/bin/bash\"]"],"Image":"sha256:3b1b0c55a47e10ea93d904fc20c39d253f9e1ad770922e8fb4af93dcec6691ce","Volumes":null,"WorkingDir":"","Entrypoint":null,"OnBuild":null,"Labels":{"maintainer":"Clement Verna \u003ccverna@fedoraproject.org\u003e"}},"created":"2020-11-12T00:25:31.334712859Z","docker_version":"19.03.12","history":[{"created":"2019-01-16T21:21:55.569693599Z","created_by":"/bin/sh -c #(nop)  LABEL maintainer=Clement Verna \u003ccverna@fedoraproject.org\u003e","empty_layer":true},{"created":"2020-04-30T23:21:44.324893962Z","created_by":"/bin/sh -c #(nop)  ENV DISTTAG=f33container FGC=f33 FBR=f33","empty_layer":true},{"created":"2020-11-12T00:25:30.976066436Z","created_by":"/bin/sh -c #(nop) ADD file:240dde03c4d9f0ad759f8d1291fb45ab2745b6a108c6164d746766239d3420ab in / "},{"created":"2020-11-12T00:25:31.334712859Z","created_by":"/bin/sh -c #(nop)  CMD [\"/bin/bash\"]","empty_layer":true}],"os":"linux","rootfs":{"type":"layers","diff_ids":["sha256:ed0c36ccfcbe08498869bb435711b2657b593806792e29582fa90f43d87b2dfb"]}}
            "##;
        let parsed = serde_json::from_str::<Schema2Image>(input);
        assert!(parsed.is_ok(), "{}", parsed.err().unwrap())
    }
}
