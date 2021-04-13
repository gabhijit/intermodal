//! Image 'mount' related functionality

use std::path::Path;

/// Mount a container Image.
///
/// Mounting a container image involves extracting the individual layers and mounting them for
/// underlying storage. We are supporting 'overlayfs' so it means we'll have to `apply_layer` for
/// every layer.
pub fn mount_container_image<P>(_reference: &str, _to_path: P) -> std::io::Result<()>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    Ok(())
}
