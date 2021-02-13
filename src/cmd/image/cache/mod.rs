//! Utilities to handle local 'blob' cache

use clap::{App, ArgMatches, SubCommand};

use crate::utils::image_blobs_cache_root;

/// API to subscribe to 'pull' subcommand
pub fn add_subcommand_clear_cache() -> App<'static, 'static> {
    SubCommand::with_name("clear-blob-cache").about("Clear local cache of downloaded Blobs.")
}

/// Actually run 'clear-blob-cache'
pub fn run_subcmd_clear_cache(_: &ArgMatches<'_>) -> std::io::Result<()> {
    log::warn!("Clearing cache of downloaded blobs. Deleting all downloaded blobs!");
    let blobs_cache_dir = image_blobs_cache_root()?;
    match std::fs::remove_dir_all(&blobs_cache_dir) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("Error '{}' in trying to delete blobs cache.'", e);
            Err(e)
        }
    }
}
