#![allow(unused_variables)]

use clap::{crate_version, App, AppSettings};

use intermodal::cmd::image;
use intermodal::image::transports;

fn main() {
    let matches = App::new("Container handling in Rust")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .version(crate_version!())
        .subcommand(image::add_subcmd_image().subcommand(image::inspect::add_subcmd_inspect()))
        .get_matches();

    println!("{:?}", matches.subcommand());

    transports::init_transports();

    #[allow(clippy::single_match)]
    match matches.subcommand() {
        ("image", Some(subcmd)) => {
            image::run_subcmd_image(subcmd);
        }
        _ => {}
    }
}
