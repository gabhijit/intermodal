use std::io;

use clap::{crate_version, App, AppSettings, Arg};
use env_logger::Env;

use intermodal_rs::cmd::image;
use intermodal_rs::image::transports;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let matches = App::new("Container handling in Rust")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .version(crate_version!())
        .arg(
            Arg::with_name("debug")
                .short("d")
                .global(true)
                .multiple(true)
                .help("Turns on verbose/debugging mode"),
        )
        .subcommand(
            image::add_subcmd_image()
                .subcommand(image::inspect::add_subcmd_inspect())
                .subcommand(image::pull::add_subcommand_pull())
                .subcommand(image::cache::add_subcommand_clear_cache()),
        )
        .get_matches();

    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or(
        match matches.occurrences_of("debug") {
            0 => "info",
            1 => "debug",
            _ => "trace",
        },
    ))
    .format_timestamp(None)
    .init();

    transports::init_transports();

    #[allow(clippy::single_match)]
    let _ = match matches.subcommand() {
        ("image", Some(ref subcmd)) => Ok(image::run_subcmd_image(subcmd).await?),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unknown Command!",
        )),
    };

    Ok(())
}
