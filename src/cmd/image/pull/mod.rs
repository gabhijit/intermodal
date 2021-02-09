//! Handling of 'pull' subcommand of 'image' command

use std::io;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::{
    image::{oci::layout, transports},
    utils::oci_image_layout_tempdir,
};

/// API to subscribe to 'pull' subcommand
pub fn add_subcommand_pull() -> App<'static, 'static> {
    SubCommand::with_name("pull")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("pull container image")
        .arg(
            Arg::with_name("name")
                .required(true)
                .help("Image name to pull")
                .index(1),
        )
        .arg(
            Arg::with_name("force")
                .help("Force pull the image.")
                .short("f")
                .long("force"),
        )
}

/// API to run 'pull' subcommand
pub async fn run_subcmd_pull(cmd: &ArgMatches<'_>) -> io::Result<()> {
    let image_name = cmd.value_of("name").unwrap();
    log::debug!("Image Name: {}", image_name);

    let force = cmd.is_present("force");

    let image_ref = transports::parse_image_name(image_name)?;
    let docker_ref = image_ref.docker_reference();

    if docker_ref.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid Image Name {}", image_name),
        ));
    }

    let name = docker_ref.as_ref().unwrap().name();
    let tag = docker_ref.as_ref().unwrap().name();
    let path = oci_image_layout_tempdir()?;

    let img_layout = layout::OCIImageLayout::new(&name, Some(&tag), Some(&path));

    if img_layout.fs_path_exists && !force {
        let errstr = format!("Local FS path for the image with name: {}, tag: {} exists. Please specify `--force` to overwrite.", name, tag);
        log::error!("{}", errstr);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, errstr));
    }

    log::debug!("Pulling the image: {}", image_name);

    let mut img = image_ref.new_image()?;

    let manifest = img.resolved_manifest().await?;

    let digest = Digest::from_bytes(&manifest.manifest);

    // Download and verify config

    // Download and verify each of the layer blobs. If the blobs are gzipped
    // unzip the blobs (Don't unzip use unzip + reader) and then verify the signature
    // as mentioned in config rootfs. If fails - fail

    // We now have everything - Write this to disk layout.

    Ok(())
}
