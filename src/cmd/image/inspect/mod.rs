use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::image::transports;

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

pub fn run_subcmd_inspect(cmd: &ArgMatches) {
    let image_name = cmd.value_of("name").unwrap();

    println!("Image Name: {}", image_name);

    if let Ok(image_ref) = transports::parse_image_name(image_name) {
        println!(
            "Valid Reference found! {}",
            image_ref.string_within_transport()
        );
    } else {
        println!("Invalid Image Name, {}", image_name);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
}
