//! Implementation of 'mount'ing image layers

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

/// API for 'mount' subcommand
pub fn add_subcommand_mount() -> App<'static, 'static> {
    SubCommand::with_name("mount")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("mount layers of a container image to create RootFS.")
        .arg(
            Arg::with_name("name")
                .required(true)
                .help("Image name to mount")
                .index(1),
        )
}

/// API to run subcommand 'mount'
///
/// Note: We'll always 'mount' the layers such that they can be 'mounted' by 'overlayFS'
pub async fn run_subcommand_mount(subcmd: &ArgMatches<'_>) -> std::io::Result<()> {
    // Find Locally 'pulled' Image. (For now let's just work with docker://<ref> paths.
    // For each of the Layers, create a directory inside some path and then
    // 1. Untar layers one by one there (creating appropriate directories as required.)
    // 2. Convert the white-outs to something that are friendly with 'overlay' FS.
    // 3. Finally create a RootFS ish path (This should be ephemeral) which can be 'unmounted'
    //    somehow. Not sure how yet.
    Ok(())
}
