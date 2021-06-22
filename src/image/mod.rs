//! Functionality for Handling Container Images
//!
//! # References
//! - Tries to implement a functionality similar to the following `Go` library
//! [Container Images Go library](https://github.com/containers/image/)

pub mod api;
pub mod docker;
pub mod manifest;
pub mod oci;
mod platform;
pub mod transports;
pub mod types;
