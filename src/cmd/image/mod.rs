use std::io;

use clap::{App, AppSettings, ArgMatches, SubCommand};

pub mod inspect;

pub fn add_subcmd_image() -> App<'static, 'static> {
    SubCommand::with_name("image")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("Command to handle container images.")
}

pub fn run_subcmd_image(subcmd: &ArgMatches) -> io::Result<()> {
    #[allow(clippy::single_match)]
    match subcmd.subcommand() {
        ("inspect", Some(inspect_subcmd)) => inspect::run_subcmd_inspect(inspect_subcmd),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown Subcommand")),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Passing No argument to this Subcommand should Fail.
    #[test]
    fn should_not_succeed() {
        let m = add_subcmd_image().get_matches_from_safe(vec!["image"]);

        assert!(m.is_err(), "{}", m.err().unwrap());
    }

    /// Passing any argument to this Subcommand should succeed.
    #[test]
    fn should_succeed() {
        let m = add_subcmd_image().get_matches_from_safe(vec!["image", "foo"]);

        assert!(m.is_err(), "{}", m.err().unwrap());
    }
}
