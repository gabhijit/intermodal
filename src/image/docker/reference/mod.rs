//! Utilities for handling Docker Image Reference
//!
//! Docker Image Reference is specified with a grammar (see `parser`).
//!
pub(crate) mod api;
mod errors;
mod parser;
pub(crate) mod types;

pub(crate) use errors::ReferenceError;
