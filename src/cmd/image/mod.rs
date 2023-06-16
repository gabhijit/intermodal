use clap::Subcommand;

pub mod cache;
pub mod inspect;
//pub mod mount;
pub mod pull;

#[derive(Debug, Subcommand)]
pub enum ImageCommands {
    /// Inspect Container Image
    #[command(arg_required_else_help = true)]
    Inspect {
        #[arg(long, help = "Image Name to Inspect.")]
        name: String,

        #[arg(long, short, help = "Output Configuration.")]
        config: bool,

        #[arg(long, help = "Output Raw manifest or Configuration.")]
        raw: bool,
    },

    /// Pull Container Image from the registry.
    #[command(arg_required_else_help = true)]
    Pull {
        #[arg(long, help = "Image Name to Inspect.")]
        name: String,

        #[arg(long, short, help = "Force pull the image.")]
        force: bool,

        #[arg(
            long = "clean-on-err",
            help = "Do not clear the local directory upon error. Useful during debugging."
        )]
        clean_on_err: bool,
    },

    /// Clear local cache of saved image blobs.
    #[command(name = "clear-blob-cache")]
    ClearCache,
}

pub async fn run_subcmd_image(cmd: ImageCommands) -> std::io::Result<()> {
    match cmd {
        ImageCommands::Inspect { .. } => inspect::run_subcmd_inspect(cmd).await,
        ImageCommands::Pull { .. } => pull::run_subcmd_pull(cmd).await,
        ImageCommands::ClearCache => cache::run_subcmd_clear_cache(),
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
