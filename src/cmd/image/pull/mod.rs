//! Handling of 'pull' subcommand of 'image' command

use std::io;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::image::api::pull_container_image;
use crate::utils::oci_images_root;

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
        .arg(
            Arg::with_name("clear-on-err")
                .help("Do not clear the local directory upon error. Useful during debugging.")
                .long("clear-on-err"),
        )
}

/// API to run 'pull' subcommand
pub async fn run_subcmd_pull(subcmd: &ArgMatches<'_>) -> io::Result<()> {
    let reference = subcmd.value_of("name").unwrap();
    let force = subcmd.is_present("force");
    let clean_on_err = subcmd.is_present("clear-on-err");

    let to_path = oci_images_root()?;

    let _ = pull_container_image(reference, to_path, force, clean_on_err).await?;

    Ok(())
}
