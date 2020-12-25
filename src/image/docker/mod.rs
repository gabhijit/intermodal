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
pub mod reference;
pub mod source;
pub mod transport;
