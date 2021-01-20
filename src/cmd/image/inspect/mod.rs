//! Handling of 'inspect' subcommand of 'image' command

use std::collections::HashMap;
use std::io;
use std::string::String;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use serde::Serialize;

use crate::image::{oci::digest::Digest, transports};

// We use references because, this will be generated from underlying 'image.inspect' struct.
// which contains 'owned' values, For our case, the underlying struct will 'outlive' this.
// We try to match the output as closely as 'skopeo inspect'
#[derive(Serialize)]
struct InspectOutput<'a> {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(rename = "Tag", skip_serializing_if = "Option::is_none")]
    tag: Option<&'a str>,

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

/// API function to subscribe handling of 'inspect' subcommands
pub fn add_subcmd_inspect() -> App<'static, 'static> {
    SubCommand::with_name("inspect")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("inspect container images")
        .arg(
            Arg::with_name("name")
                .required(true)
                .help("Image name to inspect")
                .index(1),
        )
        .arg(
            Arg::with_name("config")
                .help("output configuration")
                .short("c")
                .long("config"),
        )
        .arg(
            Arg::with_name("raw")
                .help("output raw manifest or configuration")
                .long("raw"),
        )
}

/// Run the 'inspect' subcommand asynchronously.
pub async fn run_subcmd_inspect(cmd: &ArgMatches<'_>) -> io::Result<()> {
    let image_name = cmd.value_of("name").unwrap();

    let config = cmd.is_present("config");
    let raw = cmd.is_present("raw");

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
                String::from_utf8(manifest.manifest).unwrap()
            );
        }

        if config {
            log::debug!("Getting Config for the image.");
            if raw {
                println!(
                    "Config Blob for Image '{}' : {}",
                    image_name,
                    String::from_utf8(image.config_blob().await?).unwrap()
                );
            } else {
                let inspect_data = image.inspect().await?;
                let tags = image.source_ref().get_repo_tags().await?;
                log::debug!("Tags: {:#?}", tags);

                let reference_name = match image_ref.docker_reference() {
                    Some(r) => Some(r.name()),
                    None => None,
                };

                let output = InspectOutput {
                    name: reference_name,
                    tag: None, // FIXME: Get from Docker Reference Tag
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
