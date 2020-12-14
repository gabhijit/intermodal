#![allow(unused_variables)]

use clap::{crate_version, App, AppSettings};

use intermodal::cmd;

fn main() {
    let matches = App::new("Container handling in Rust")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .version(crate_version!())
        .subcommand(
            cmd::image::add_subcmd_image().subcommand(cmd::image::inspect::add_subcmd_inspect()),
        )
        .get_matches();

    println!("{:?}", matches);
}
