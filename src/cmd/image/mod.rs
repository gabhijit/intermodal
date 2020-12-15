use clap::{App, AppSettings, SubCommand};

pub mod inspect;

pub fn add_subcmd_image() -> App<'static, 'static> {
    SubCommand::with_name("image")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("Command to handle container images.")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_succeed() {
        let m = add_subcmd_image().get_matches_from_safe(vec!["image"]);

        assert!(m.is_err());
    }
}
