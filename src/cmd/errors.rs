#![allow(dead_code)]

//! Result Type and Error values associated with running commands

use std::error::Error;
use std::fmt;
use std::result::Result;

pub type CmdResult<T> = Result<T, CmdError>;

#[derive(Debug)]
pub enum CmdError {
    UnknownError(String),
    ImageInspectCmdError(String),
}

impl fmt::Display for CmdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CmdError::UnknownError(ref msg) => write!(formatter, "UnknownError: {}", msg),
            CmdError::ImageInspectCmdError(ref msg) => write!(formatter, "UnknownError: {}", msg),
        }
    }
}

impl Error for CmdError {}
