
#![allow(unused_variables)]


use clap::{crate_version, App, AppSettings};


fn main() {

    let matches = App::new("intmod")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .version(crate_version!())
        .get_matches();
    println!("Hello World! {:?}", matches);

}
