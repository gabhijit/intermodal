#![warn(unused_variables)]
use std::io;

use clap::{crate_version, App, AppSettings, Arg};
use env_logger::Env;

use intermodal::cmd::image;
use intermodal::image::transports;

#[tokio::main]
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
        .subcommand(image::add_subcmd_image().subcommand(image::inspect::add_subcmd_inspect()))
        .get_matches();

    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or(
        match matches.occurrences_of("debug") {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        },
    ))
    .format_timestamp(None)
    .init();

    transports::init_transports();

    #[allow(clippy::single_match)]
    let _ = match matches.subcommand() {
        ("image", Some(subcmd)) => {
            image::run_subcmd_image(subcmd).await?;
            Ok(())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unknown Command!",
        )),
    };

    Ok(())
}
