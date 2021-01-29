//! # intermodal
//!
//! intermodal is an implementation of container handling in rust.
//!
//! ## About
//!
//! The main idea is to provide programmable APIs for the following actions broadly related to
//! hadling containers
//!
//! - Manipulating Container Images (inspect, fetch)
//! - An OCI Compliant Runtime in Rust (spawn containers)
//! - Implementation of a CRI Server, so this whole thing can run behind a 'kubelet'
//!
//! Also, implementation of utilities for handling containers, images etc. (A reference
//! implementation using the above library functionality.)
//!
//! Goal is to have a full featured implementation supporting Cgroups, Namespaces, seccomp etc.
//!
//! Initial target is for Linux systems mainly.

pub mod cmd;
pub mod image;
pub mod utils;

// FIXME: Not sure how to use below, but better to get started.
#[macro_export]
macro_rules! log_err_obj {
    ($et:expr, $($arg:tt)*) => {
        let errstr = format!($($arg)*);
        log::error!("{}", errstr);
        $et(errst)
    }
}

#[macro_export]
macro_rules! log_err_return {
    ($et:expr, $($arg:tt)*) => {
        let errstr = format!($($arg)*);
        log::error!("{}", errstr);
        return Err($et(errstr));
    };
}
