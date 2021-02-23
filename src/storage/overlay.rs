//! Functionality related to handling 'overlay' file-system

use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::{image::oci::digest::Digest, utils::storage_root_for_fs};

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
pub fn apply_layer<P: AsRef<Path>>(
    digest: &Digest,
    layer: P,
    base_path: Option<&PathBuf>,
) -> std::io::Result<()> {
    let mut layer_path: PathBuf;
    if base_path.is_none() {
        layer_path = layers_base_path()?;
    } else {
        layer_path = PathBuf::from(base_path.unwrap())
    }

    let _ = layer_path.push(format!("{}/{}", digest.algorithm(), digest.hex_digest()));

    if !layer_path.exists() {
        std::fs::create_dir_all(layer_path)?;
    }

    let reader = BufReader::new(std::fs::File::open(layer)?);
    let gz_reader = flate2::bufread::GzDecoder::new(reader);
    let mut tar_reader = tar::Archive::new(gz_reader);

    let entries = tar_reader.entries()?;

    for entry in entries {
        println!("{:#?}", entry?.header());
    }

    Ok(())
}

// TODO: Implement an API - cleanup layer
// Given a layer digest, cleans up the given layer. deletes everything under the directory for the
// layer.

#[cfg(test)]
mod tests {

    use super::*;
    use crate::image::oci::digest::Digest;

    // FIXME: Hard coded
    #[test]
    fn test_apply_layer() {
        assert!(true);
        let digest = Digest::new_from_str(
            "sha256:5c4213be9af904dd74649d250f22023f532b2f9179ffcb15260b5eaa10d7a3b4",
        )
        .unwrap();
        let blobpath =
            "/home/gabhijit/.local/share/intmod/images/docker.io/library/busybox/1.32/blobs/sha256/5c4213be9af904dd74649d250f22023f532b2f9179ffcb15260b5eaa10d7a3b4";

        let r = apply_layer(&digest, blobpath, Some(&PathBuf::from("/tmp")));
        assert!(r.is_ok());
    }
}
