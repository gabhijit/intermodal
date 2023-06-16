use std::io;

use clap::{Parser, Subcommand};
use env_logger::Env;

use intermodal_rs::cmd::image::{self, ImageCommands};
use intermodal_rs::image::transports;

#[derive(Debug, Parser)]
#[command(name = "intermodal")]
#[command(about = "Container handling in Rust.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Handle container images.
    #[command(arg_required_else_help = true)]
    Image {
        #[command(subcommand)]
        image_commands: ImageCommands,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    eprintln!("cli: {:?}", cli);
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or(match cli.debug {
        0 => "info",
        1 => "debug",
        _ => "trace",
    }))
    .format_timestamp(None)
    .init();

    transports::init_transports();

    match cli.commands {
        Commands::Image { image_commands } => image::run_subcmd_image(image_commands).await,
    }
}
