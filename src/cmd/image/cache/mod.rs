//! Utilities to handle local 'blob' cache

use crate::utils::image_blobs_cache_root;

/// Actually run 'clear-blob-cache'
pub fn run_subcmd_clear_cache() -> std::io::Result<()> {
    log::warn!("Clearing cache of downloaded blobs. Deleting all downloaded blobs!");
    let blobs_cache_dir = image_blobs_cache_root()?;
    match std::fs::remove_dir_all(blobs_cache_dir) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("Error '{}' in trying to delete blobs cache.'", e);
            Err(e)
        }
    }
}
