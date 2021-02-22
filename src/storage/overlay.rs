//! Functionality related to handling 'overlay' file-system

use std::path::PathBuf;

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
pub fn apply_layer(digest: &Digest, layer: &PathBuf) -> std::io::Result<()> {
    let mut layer_path = layers_base_path()?;

    let _ = layer_path.push(format!("{}/{}", digest.algorithm(), digest.hex_digest()));

    if !layer_path.exists() {
        std::fs::create_dir_all(layer_path)?;
    }
    Ok(())
}
