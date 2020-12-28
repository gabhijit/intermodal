//! Handling of 'inspect' subcommand of 'image' command

use std::io;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::image::transports;

/// API function to subscribe handling of 'inspect' subcommands
pub fn add_subcmd_inspect() -> App<'static, 'static> {
    SubCommand::with_name("inspect")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("inspect container images")
        .arg(
            Arg::with_name("name")
                .required(true)
                .help("Image name to inspect")
                .index(1),
        )
        .arg(
            Arg::with_name("config")
                .help("output configuration")
                .short("c")
                .long("config"),
        )
        .arg(
            Arg::with_name("raw")
                .help("output raw manifest or configuration")
                .long("raw"),
        )
}

/// Run the 'inspect' subcommand asynchronously.
pub async fn run_subcmd_inspect(cmd: &ArgMatches<'_>) -> io::Result<()> {
    let image_name = cmd.value_of("name").unwrap();

    log::debug!("Image Name: {}", image_name);

    if let Ok(image_ref) = transports::parse_image_name(image_name) {
        log::debug!(
            "Valid Reference found! {}",
            image_ref.string_within_transport()
        );
        let _ = image_ref.new_image_source();
        Ok(())
    } else {
        let err = format!("Invalid Image Name: {}", image_name);

        log::error!("{}", &err);
        Err(io::Error::new(io::ErrorKind::InvalidInput, err))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::image::transports;

    #[test]
    fn test_subcommand_inspect_no_name() {
        let m = add_subcmd_inspect().get_matches_from_safe(vec!["inspect"]);
        assert!(m.is_err());
    }

    #[test]
    fn test_subcommand_image_name() {
        let m = add_subcmd_inspect()
            .get_matches_from_safe(vec!["inspect", "fedora"])
            .unwrap();

        assert_eq!(m.value_of("name"), Some("fedora"));
    }

    #[test]
    fn test_unsupported_flag() {
        let m = add_subcmd_inspect().get_matches_from_safe(vec!["inspect", "--war"]);

        assert!(m.is_err());
    }

    #[tokio::test]
    async fn test_subcommand_run_success() {
        transports::init_transports();
        let m = add_subcmd_inspect()
            .get_matches_from_safe(vec!["inspec", "docker://fedora"])
            .unwrap();
        let name = m.value_of("name").unwrap();

        assert_eq!(name, "docker://fedora");

        let result = run_subcmd_inspect(&m).await;
        assert!(result.is_ok());
    }
}
