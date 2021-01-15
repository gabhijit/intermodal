//! Utilities for handling Platforms for Images

use crate::image::oci::spec_v1::Platform;

/// Function that returns OCI Image Spec v1 -> Platform structure.
///
/// Whenever we have a list of Manifests (docker) or a Manifest Index (OCI), to chose the right
/// manifest for the current platform, the current platform/Os needs to be determined. Plus there
/// are naming differences between docker image names and reported architecture names (eg. 'x86_64'
/// vs. 'amd64', 'arm64' vs 'aarch64' etc. All those differences are abstracted out and returns
/// names that the `platform` field in the image manifest will like.
pub(crate) fn get_os_platform() -> Platform {
    let architecture = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "arm" => "arm",
        "aarch64" => "arm64",
        _ => std::env::consts::ARCH,
    }
    .to_string();

    let variant = match &architecture as &str {
        "arm64" => Some("v8".to_string()),
        "arm" => Some("v7".to_string()), // FIXME: Determine properly.
        _ => None,
    };

    Platform {
        os: std::env::consts::OS.to_string(),
        architecture,
        variant,
        os_version: None,
        os_features: None,
    }
}
