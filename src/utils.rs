//! Utility functions used by trait and possibly useful outside as well.

use std::path::PathBuf;

use directories::ProjectDirs;

const QUALIFIER: &str = "io";
const ORGANIZATION: &str = "";
const APPLICATION: &str = "intmod";

/// Get's the image 'blobs' cache root path
///
/// When Blobs are downloaded (via http(s) say.), the blobs are stored at this location 'after' the
/// digest is verified. The actual 'blob' path then can be stored in a cache by an App (inside some
/// kind of digest->path map).
///
/// Each blob will be saved at a path like `/cache_root/<alg>/<digest>` Path. It is safe to assume
/// that if there's a path corresponding to a 'blob' here, the contents of the 'blob' do indeed
/// match the checksum.
///
/// Note: This path is different from the `blobs` directory inside an OCI image layout. For OCI
/// images, the `blobs` directory is maintained per image. All blobs for a particular image
/// (including those for different tags will be contained in that directory.) The blobs in these
/// cache are not related to each other. They serve just as `cache`.
///
pub fn image_blobs_cache_root() -> std::io::Result<PathBuf> {
    let mut blobs_cache_dir = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(p) => PathBuf::from(p.cache_dir()),
        None => std::env::temp_dir(),
    };

    let _ = blobs_cache_dir.push("blobs");

    if !blobs_cache_dir.exists() {
        log::debug!("The Parent Cache directory does not exist. Creating.");
        std::fs::create_dir_all(&blobs_cache_dir)?;
    }

    Ok(blobs_cache_dir)
}

/// Cleans up the image 'blobs' cache root path directory and everything underneath.
pub fn image_blobs_cache_clear() -> std::io::Result<()> {
    let blobs_cache_dir = image_blobs_cache_root()?;
    match std::fs::remove_dir_all(&blobs_cache_dir) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("Error '{}' in trying to delete blobs cache.'", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_blobs_cache_dir() {
        let r = image_blobs_cache_root();
        assert!(r.is_ok());
    }
}
