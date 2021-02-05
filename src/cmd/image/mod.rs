use std::io;

use clap::{App, AppSettings, ArgMatches, SubCommand};

pub mod inspect;
pub mod pull;

/// Command line Parsing for 'image' subcommand
pub fn add_subcmd_image() -> App<'static, 'static> {
    SubCommand::with_name("image")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("Command to handle container images.")
}

/// Run 'image' subcommand asynchronously
pub async fn run_subcmd_image(subcmd: &ArgMatches<'_>) -> io::Result<()> {
    #[allow(clippy::single_match)]
    match subcmd.subcommand() {
        ("inspect", Some(inspect_subcmd)) => Ok(inspect::run_subcmd_inspect(inspect_subcmd).await?),
        ("pull", Some(pull_subcmd)) => Ok(pull::run_subcmd_pull(pull_subcmd).await?),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown Subcommand")),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use inspect::*;

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

    /// Test the 'inspect' subcommand
    #[tokio::test]
    async fn test_inspect_subcommand_run_should_succeed_with_error() {
        let m = add_subcmd_image()
            .subcommand(add_subcmd_inspect())
            .get_matches_from_safe(vec!["image", "inspect", "docker://docker.io/fedora"])
            .unwrap();

        let result = run_subcmd_image(&m).await;
        assert!(result.is_ok());
    }
}
