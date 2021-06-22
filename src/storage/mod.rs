//! Related to handling of 'storage' of images.
//!
//! When we want to do something useful with the 'images', we need to 'mount' those images to be
//! able to work with those images. How the images can be 'mounted' is determined amongst other
//! things by the underlying filesystem on which we want to mount the image. This involves, what is
//! called as 'applying' the layer as described in [layer specification][1].
//!
//! [1]: https://github.com/opencontainers/image-spec/blob/master/layer.md
//!
//! Clearly this will be different for different file systems. We'll initially support 'overlay'
//! file-system.
//!
//! This will be the layout of the directories
//!
//! <~/.local/share/intmod>/storage/<fstype>/layers/ - This directory contains the layers of the
//! image that are extracted. Each layer contains the directory below the path mentioned above for
//! it's digest <algorithm/hex-digest/> This is obtained from the `root_fs.layers` of the Image
//! configuration.
//!
//! <.../intmod/storage<fstype>/mounts/ - The directory where the layers are mounted inside the
//! mounts/ directory above the following layout is present per mount.
//!
//! <sha256sum>/ -- This is 'mount ID' which will be container ID when the 'mount' corresponds to a
//! container that is running or an ID generated when user issues a 'mount' command.
//!
//! The above directory has Following two entries
//!
//! rootfs/ - Directory that the container runtime will use as a root filesystem
//! container.json - Optional / Empty file containing the runtime config (this may change.) if the
//! 'mount' corresponds to a running container.
//!
//! This module is responsible for handling all the storage related details (losely corresponds to
//! 'docker storage drivers').
//!

pub mod overlay;
