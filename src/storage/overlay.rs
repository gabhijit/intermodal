//! Functionality related to handling 'overlay' file-system

use std::ffi::CString;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use crate::{image::oci::digest::Digest, utils::storage_root_for_fs};

// Constants specific to overlay FS
const WHITEOUT_PREFIX: &str = ".wh.";
const WHITEOUT_OPAQUE: &str = ".wh..wh..opq";
const XATTR_OVERLAY_FS_OPAQUE_KEY: &str = "trusted.overlay.opaque";
const XATTR_OVERLAY_FS_OPAQUE_VAL: &[u8; 1] = b"y";

/// Returns the Path to the 'layers' directory.
pub fn layers_base_path() -> std::io::Result<PathBuf> {
    let mut layers_base_path = storage_root_for_fs("overlay")?;
    let _ = layers_base_path.push("layers");
    if !layers_base_path.exists() {
        std::fs::create_dir_all(&layers_base_path)?;
    }
    Ok(layers_base_path)
}

/// 'apply' the given layer to the FS path.
///
/// For the 'overlay' filesystem, this involves, extracting the tar files and handling the
/// whiteouts.
pub fn apply_layer<P: AsRef<Path> + std::fmt::Debug>(
    digest: &Digest,
    layer: P,
    base_path: Option<&PathBuf>,
    lower: &str,
) -> std::io::Result<()> {
    let mut layer_path: PathBuf;
    if base_path.is_none() {
        layer_path = layers_base_path()?;
    } else {
        layer_path = PathBuf::from(base_path.unwrap())
    }

    log::debug!("Applying Layer: `{:?}`", layer);

    let _ = layer_path.push(format!("{}/{}", digest.algorithm(), digest.hex_digest()));

    // Create the directory identified by the checksum
    if !layer_path.exists() {
        std::fs::create_dir_all(&layer_path)?;
    }

    log::trace!("Creating The files and directories for the layer!");

    // create the 'diff' directory - This is where 'rootFS' will be mounted
    log::trace!("Creating 'diff' directory to extract the layer.");
    let mut diff_path = PathBuf::from(&layer_path);
    let _ = diff_path.push("diff");
    std::fs::create_dir_all(&diff_path)?;

    // Writes the 'lower' file as per docker's overlay2
    //
    // Also creates 'work' dir required by 'overalyfs'
    if !lower.is_empty() {
        log::trace!("Generating 'lower' file and 'work' directory.");
        let mut lower_path = PathBuf::from(&layer_path);
        let _ = lower_path.push("lower");
        let mut f = std::fs::File::create(lower_path)?;
        f.write_all(lower.as_bytes())?;
        f.sync_all()?;

        let mut workdir_path = PathBuf::from(&layer_path);
        let _ = workdir_path.push("work");
        std::fs::create_dir(workdir_path)?;
    }

    log::trace!("Applying entries in the Layer Tar!");
    let reader = BufReader::new(std::fs::File::open(layer)?);
    let gz_reader = flate2::bufread::GzDecoder::new(reader);
    let mut tar_reader = tar::Archive::new(gz_reader);

    let entries = tar_reader.entries()?;

    for entry in entries {
        let mut entry = entry?;
        let is_whiteout = entry
            .path()
            .unwrap()
            .to_str()
            .unwrap()
            .contains(WHITEOUT_PREFIX);
        if is_whiteout {
            // Handle whiteout will do everything to
            // 1. 'write' the entry to the FS if required
            // 2. 'create' char(0, 0) device at the path.
            // 3. set `xattr` etc.
            handle_whiteout(&diff_path, &entry)?;
        } else {
            // Not a white-out simply write the entry to the path.
            let _ = entry.unpack_in(&diff_path)?;
        }
    }

    Ok(())
}

// Handles the whiteout entry for the Overlay FS
//
// Ref: https://www.kernel.org/doc/html/latest/filesystems/overlayfs.html
//
fn handle_whiteout<'a, P, R>(base: P, entry: &tar::Entry<'a, R>) -> std::io::Result<()>
where
    P: AsRef<Path>,
    R: 'a + std::io::Read,
{
    // An Opaque whiteout entry.
    let entry_path = entry.path().unwrap();
    log::trace!("Handling whiteout Entry: {:?}", entry_path);

    if entry_path.ends_with(WHITEOUT_OPAQUE) {
        log::trace!("Entry is an opaque entry, applying 'xattr'.");
        let mut components = entry_path.components();
        if let Some(_) = components.next_back() {
            // Last is consumed. use whatever remains as a path.
            let joined = base.as_ref().join(components.as_path());
            std::fs::create_dir_all(&joined)?;
            xattr::set(
                joined,
                XATTR_OVERLAY_FS_OPAQUE_KEY,
                XATTR_OVERLAY_FS_OPAQUE_VAL,
            )?;
        }
    } else {
        log::trace!("Entry is a simple whiteout entry. Creating a char device for the entry!");
        let mknod_path_str = entry_path.to_str().unwrap().replace(WHITEOUT_PREFIX, "");
        let mknod_path = Path::new(&mknod_path_str);
        let joined_path = base.as_ref().join(mknod_path);
        let joined_str = joined_path.to_str().unwrap();
        let joined_cstr = CString::new(joined_str)?;

        unsafe {
            libc::mknod(joined_cstr.as_ptr(), libc::S_IFCHR, libc::makedev(0, 0));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::image::api::pull_container_image;
    use crate::image::oci::digest::Digest;
    use crate::image::oci::layout::OCIImageLayout;

    // Pulls a busybox image for testing. Note: the passed 'Path' should be a `tempdir` Path, which
    // can be cleaned automatically when the test case exits.
    async fn pull_busybox_image_for_test(
        to_path: &std::path::Path,
    ) -> std::io::Result<OCIImageLayout> {
        pull_container_image("docker://busybox:1.32", to_path, true, true).await
    }

    #[tokio::test]
    async fn test_apply_layer() {
        let digest = Digest::new_from_str(
            "sha256:5c4213be9af904dd74649d250f22023f532b2f9179ffcb15260b5eaa10d7a3b4",
        )
        .unwrap();

        let prefix = "layer_test";
        let pull_tempdir = tempdir::TempDir::new(prefix).unwrap();
        let r = pull_busybox_image_for_test(pull_tempdir.path()).await;
        assert!(r.is_ok());

        let image_layout = r.unwrap();

        let mut blobpath = image_layout.image_fs_path();
        blobpath.push(format!(
            "{}/{}/{}",
            "blobs",
            digest.algorithm(),
            digest.hex_digest()
        ));

        let layout_tempdir = tempdir::TempDir::new(prefix).unwrap();
        let r = apply_layer(
            &digest,
            blobpath,
            Some(&PathBuf::from(layout_tempdir.path())),
            "",
        );
        assert!(r.is_ok());
    }
}
