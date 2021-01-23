//! Docker Image Handling
//!
//! - `client` - Docker Client for talking with docker repositories
//! - `dst` - Image Destination that is Docker specific (used in Image copy)
//! - `src` - Image Source that is Docker specific (used in Image copy)
//! - `reference` - Handling for Docker references
//!
//! References:
//! [Docker Implementation](https://github.com/containers/image/tree/master/docker)

pub mod client;
pub mod dst;
pub mod errors;
pub mod image;
mod manifest;
pub mod reference;
pub mod source;
pub mod transport;

pub(crate) const MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST: &str =
    "application/vnd.docker.distribution.manifest.v2+json";
pub(crate) const MEDIA_TYPE_DOCKER_V2_LIST: &str =
    "application/vnd.docker.distribution.manifest.list.v2+json";

#[cfg(test)]
mod testdata;
#[cfg(test)]
mod tests;
