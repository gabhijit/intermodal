use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::image::oci::digest::Digest;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Empty {}

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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
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
    pub exposed_ports: Option<HashMap<String, Empty>>,

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
    pub volumes: Option<HashMap<String, Empty>>,

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

    pub diff_ids: Vec<Digest>,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_image_index_ok() {
        // Reference: https://github.com/opencontainers/image-spec/blob/master/image-index.md
        let input = r##"{ "schemaVersion": 2, "manifests": [ { "mediaType": "application/vnd.oci.image.manifest.v1+json", "size": 7143, "digest": "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f", "platform": { "architecture": "ppc64le", "os": "linux" } }, { "mediaType": "application/vnd.oci.image.manifest.v1+json", "size": 7682, "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270", "platform": { "architecture": "amd64", "os": "linux" } } ], "annotations": { "com.example.key1": "value1", "com.example.key2": "value2" } }"##;
        let parsed = serde_json::from_str::<Index>(input);
        assert!(parsed.is_ok(), "{}", parsed.err().unwrap());
    }

    #[test]
    fn test_image_manifest_ok() {
        // Reference: https://github.com/opencontainers/image-spec/blob/master/manifest.md
        let input = r##"{ "schemaVersion": 2, "config": { "mediaType": "application/vnd.oci.image.config.v1+json", "size": 7023, "digest": "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7" }, "layers": [ { "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip", "size": 32654, "digest": "sha256:9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0" }, { "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip", "size": 16724, "digest": "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b" }, { "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip", "size": 73109, "digest": "sha256:ec4b8955958665577945c89419d1af06b5f7636b4ac3da7f12184802ad867736" } ], "annotations": { "com.example.key1": "value1", "com.example.key2": "value2" } }"##;
        let parsed = serde_json::from_str::<Manifest>(input);
        assert!(parsed.is_ok(), "{}", parsed.err().unwrap());
    }

    #[test]
    fn test_image_config_ok() {
        // Reference: https://github.com/opencontainers/image-spec/blob/master/config.md
        let input = r##"{ "created": "2015-10-31T22:22:56.015925234Z", "author": "Alyssa P. Hacker <alyspdev@example.com>", "architecture": "amd64", "os": "linux", "config": { "User": "alice", "ExposedPorts": { "8080/tcp": {} }, "Env": [ "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin", "FOO=oci_is_a", "BAR=well_written_spec" ], "Entrypoint": [ "/bin/my-app-binary" ], "Cmd": [ "--foreground", "--config", "/etc/my-app.d/default.cfg" ], "Volumes": { "/var/job-result-data": {}, "/var/log/my-app-logs": {} }, "WorkingDir": "/home/alice", "Labels": { "com.example.project.git.url": "https://example.com/project.git", "com.example.project.git.commit": "45a939b2999782a3f005621a8d0f29aa387e1d6b" } }, "rootfs": { "diff_ids": [ "sha256:c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1", "sha256:5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef" ], "type": "layers" }, "history": [ { "created": "2015-10-31T22:22:54.690851953Z", "created_by": "/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /" }, { "created": "2015-10-31T22:22:55.613815829Z", "created_by": "/bin/sh -c #(nop) CMD [\"sh\"]", "empty_layer": true } ] }"##;

        let parsed = serde_json::from_str::<ImageConfig>(input);
        assert!(parsed.is_ok(), "{}", parsed.err().unwrap());
    }
}
