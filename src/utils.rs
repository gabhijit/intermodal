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

/// Get's the Local Path for OCI Images.
///
/// Local images are stored in a directory on the FS. The images are stored using a Layout
/// recommended in OCI Spec:
/// https://github.com/opencontainers/image-spec/blob/master/image-layout.md.
/// This API is used to get a Path to the local directory containing the root of all 'locally'
/// available Images stored in OCI Format. The images themselves are stored inside a directory
/// identified by the image name eg. Let's say there's an image called 'fedora', the way this will
/// be stored on the local directory is as follows -
/// <OCI-IMAGES-ROOT>/fedora/<IMAGE-LAYOUT>
///
/// Of the above, <OCI-IMAGES-ROOT> path is returned by the current function.
pub fn oci_images_root() -> std::io::Result<PathBuf> {
    let mut images_root_dir = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(p) => p.data_local_dir().to_path_buf(),
        None => {
            log::warn!("No Local Data Directory found, using temporary directory.");
            std::env::temp_dir()
        }
    };

    let _ = images_root_dir.push("images");

    if !images_root_dir.exists() {
        log::debug!("Images Root Directory does not exist. Creating.");
        std::fs::create_dir_all(&images_root_dir)?;
    }

    Ok(images_root_dir)
}

/// Get's the 'storage' root path for the given filesystem.
///
/// See `storage/mod.rs` for the detail.
pub fn storage_root_for_fs(fs: &str) -> std::io::Result<PathBuf> {
    let mut storage_root_dir = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(p) => p.data_local_dir().to_path_buf(),
        None => {
            log::warn!("No Local Data Directory found, using temporary directory.");
            std::env::temp_dir()
        }
    };

    let _ = storage_root_dir.push("storage");

    let _ = storage_root_dir.push(fs);

    if !storage_root_dir.exists() {
        log::debug!(
            "{}",
            format!("Creating Storage Root directory for : {}", fs)
        );
        std::fs::create_dir_all(&storage_root_dir)?;
    }

    Ok(storage_root_dir)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_blobs_cache_dir() {
        let r = image_blobs_cache_root();
        assert!(r.is_ok());
    }

    #[test]
    fn test_get_oci_images_root() {
        let r = oci_images_root();
        assert!(r.is_ok());
    }
}
