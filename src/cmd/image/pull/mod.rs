//! Handling of 'pull' subcommand of 'image' command

use std::io;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

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
}

/// API to run 'pull' subcommand
pub async fn run_subcmd_pull(cmd: &ArgMatches<'_>) -> io::Result<()> {
    // Parse the reference.

    // Get the image name and tag and see if there exists this directory
    // on local FS. If so warn and exit unless --force is provided.

    // Download the manifest (if it's a list manifest, downlod resolved_manifest.

    // Download and verify config

    // Download and verify each of the layer blobs. If the blobs are gzippped
    // unzip the blobs (Don't unzip use unzip + reader) and then verify the signature
    // as mentioned in config rootfs. If fails - fail

    // We now have everything - Write this to disk layout.

    Ok(())
}
