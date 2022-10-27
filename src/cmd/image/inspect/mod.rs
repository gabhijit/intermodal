//! Handling of 'inspect' subcommand of 'image' command

use std::collections::HashMap;
use std::io;
use std::string::String;

use serde::Serialize;

use crate::cmd::image::ImageCommands;
use crate::image::{oci::digest::Digest, transports};

// We use references because, this will be generated from underlying 'image.inspect' struct.
// which contains 'owned' values, For our case, the underlying struct will 'outlive' this.
// We try to match the output as closely as 'skopeo inspect'
#[derive(Serialize)]
struct InspectOutput<'a> {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(rename = "Tag", skip_serializing_if = "Option::is_none")]
    tag: Option<String>,

    #[serde(rename = "Digest")]
    digest: &'a str,

    #[serde(rename = "RepoTags")]
    repo_tags: &'a Vec<String>,

    #[serde(rename = "Created")]
    created: &'a str,

    #[serde(rename = "DockerVersion")]
    docker_version: &'a str,

    #[serde(rename = "Labels")]
    labels: &'a HashMap<String, String>,

    #[serde(rename = "Architecture")]
    architecture: &'a str,

    #[serde(rename = "Os")]
    os: &'a str,

    #[serde(rename = "Layers")]
    layers: &'a Vec<String>,

    #[serde(rename = "Env")]
    env: &'a Vec<String>,
}

/// Run the 'inspect' subcommand asynchronously.
pub async fn run_subcmd_inspect(cmd: ImageCommands) -> io::Result<()> {
    if let ImageCommands::Inspect {
        name: ref image_name,
        config,
        raw,
    } = cmd
    {
        log::debug!("Image Name: {}", image_name);

        if let Ok(image_ref) = transports::parse_image_name(image_name) {
            log::debug!(
                "Valid Reference found! {}",
                image_ref.string_within_transport()
            );

            let mut image = image_ref.new_image()?;

            log::debug!("calling get_manifest");
            let manifest = image.manifest().await?;

            let digeststr = Digest::from_bytes(&manifest.manifest).to_string();

            if raw {
                println!(
                    "Manifest for {}: {}",
                    image_name,
                    std::str::from_utf8(&manifest.manifest).unwrap()
                );
            }

            if config {
                log::debug!("Getting Config for the image.");
                if raw {
                    println!(
                        "Config Blob for Image '{}' : {}",
                        image_name,
                        std::str::from_utf8(&image.config_blob().await?).unwrap()
                    );
                } else {
                    let inspect_data = image.inspect().await?;
                    let tags = image.source_ref().get_repo_tags().await?;
                    log::debug!("Tags: {:#?}", tags);

                    let docker_ref = image_ref.docker_reference();

                    let mut reference_name: Option<String> = None;
                    let mut reference_tag: Option<String> = None;
                    if docker_ref.is_some() {
                        reference_name = Some(docker_ref.as_ref().unwrap().name());
                        reference_tag = Some(docker_ref.as_ref().unwrap().tag());
                    }

                    let output = InspectOutput {
                        name: reference_name,
                        tag: reference_tag,
                        digest: &digeststr,
                        repo_tags: &tags,
                        created: &inspect_data.created,
                        docker_version: &inspect_data.docker_version,
                        labels: &inspect_data.labels,
                        architecture: &inspect_data.architecture,
                        os: &inspect_data.os,
                        layers: &inspect_data.layers,
                        env: &inspect_data.env,
                    };
                    println!("{}", serde_json::to_string_pretty(&output).unwrap());
                }
            }

            Ok(())
        } else {
            let err = format!("Invalid Image Name: {}", image_name);
            log::error!("{}", &err);
            Err(io::Error::new(io::ErrorKind::InvalidInput, err))
        }
    } else {
        let err = format!("Invalid Command: {:?}", cmd);
        log::error!("{}", &err);
        Err(io::Error::new(io::ErrorKind::InvalidInput, err))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::image::transports;

    #[test]
    fn test_subcommand_inspect_no_name() {
        let m = add_subcmd_inspect().get_matches_from_safe(vec!["inspect"]);
        assert!(m.is_err());
    }

    #[test]
    fn test_subcommand_image_name() {
        let m = add_subcmd_inspect()
            .get_matches_from_safe(vec!["inspect", "fedora"])
            .unwrap();

        assert_eq!(m.value_of("name"), Some("fedora"));
    }

    #[test]
    fn test_unsupported_flag() {
        let m = add_subcmd_inspect().get_matches_from_safe(vec!["inspect", "--war"]);

        assert!(m.is_err());
    }

    #[tokio::test]
    async fn test_subcommand_run_success() {
        transports::init_transports();
        let m = add_subcmd_inspect()
            .get_matches_from_safe(vec!["inspec", "docker://fedora"])
            .unwrap();
        let name = m.value_of("name").unwrap();

        assert_eq!(name, "docker://fedora");

        let result = run_subcmd_inspect(&m).await;
        assert!(result.is_ok());
    }
}
