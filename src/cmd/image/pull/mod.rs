//! Handling of 'pull' subcommand of 'image' command

use std::io;

use crate::cmd::image::ImageCommands;
use crate::image::api::pull_container_image;
use crate::utils::oci_images_root;

/// API to run 'pull' subcommand
pub async fn run_subcmd_pull(subcmd: ImageCommands) -> io::Result<()> {
    if let ImageCommands::Pull {
        name: ref reference,
        force,
        clean_on_err,
    } = subcmd
    {
        let to_path = oci_images_root()?;

        let _ = pull_container_image(reference, to_path, force, clean_on_err).await?;

        Ok(())
    } else {
        Ok(())
    }
}
