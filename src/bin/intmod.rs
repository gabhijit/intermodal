#![allow(unused_variables)]

use clap::{crate_version, App, AppSettings};

use intermodal::image;

fn main() {
    let matches = App::new("intmod")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .version(crate_version!())
        .subcommand(image::cmd::add_subcmd_image())
        .get_matches();
    println!("Hello World! {:?}", matches);
}
