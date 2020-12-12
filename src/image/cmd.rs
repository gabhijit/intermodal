use clap::{App, AppSettings, Arg, SubCommand};

pub fn add_subcmd_image() -> App<'static, 'static> {
    SubCommand::with_name("image")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("Command to handle container images.")
        .subcommand(
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
                ),
        )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_subcommand_image_no_name() {
        let a = add_subcmd_image();
        let m = a.get_matches_from_safe(vec!["image", "inspect"]);

        assert!(m.is_err());
    }

    #[test]
    fn test_subcommand_image_name() {
        let m = add_subcmd_image()
            .get_matches_from_safe(vec!["image", "inspect", "fedora"])
            .unwrap();
        let (subcmd_name, subcmd) = m.subcommand();
        let m = subcmd.unwrap();

        assert_eq!(subcmd_name, "inspect");
        assert_eq!(m.value_of("name"), Some("fedora"));
    }

    #[test]
    fn test_unsupported_flag() {
        let m = add_subcmd_image().get_matches_from_safe(vec!["image", "inspect", "--war"]);

        assert!(m.is_err());
    }
}
