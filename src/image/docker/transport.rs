#![allow(clippy::redundant_allocation)]

//! Implementation of Docker Transport

use std::boxed::Box;
use std::error::Error as StdError;
use std::fmt;
use std::string::String;

use crate::image::docker::reference::api::parse;

use crate::image::types::errors::ImageError;
use crate::image::types::{ImageReference, ImageResult, ImageTransport};

pub(crate) static DOCKER_TRANSPORT_NAME: &str = "docker";

pub(in crate::image) fn get_docker_transport() -> (String, Box<dyn ImageTransport + Send + Sync>) {
    (
        String::from(DOCKER_TRANSPORT_NAME),
        Box::new(DockerTransport::new()),
    )
}

/// A Structure implementing Docker Transport.
///
/// Currently this structure does not have any fields, but only used as a place-holder for
/// implementing the `ImageReference` trait.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct DockerTransport {}

impl DockerTransport {
    pub(crate) fn new() -> Self {
        DockerTransport {}
    }
}

impl ImageTransport for DockerTransport {
    fn name(&self) -> String {
        String::from(DOCKER_TRANSPORT_NAME)
    }

    fn parse_reference(&self, reference: &str) -> ImageResult<Box<dyn ImageReference>> {
        let dslash = reference.find("//");

        if dslash.is_none() {
            let errstr = format!(
                "Docker Reference String '{}' does not start with '//'.",
                reference
            );
            log::error!("{}", &errstr);
            return Err(ImageError::new().with(TransportError(errstr)));
        }

        let tokens: Vec<&str> = reference.split("//").collect();

        if tokens.len() != 2 {
            let errstr = format!(
                "Input Image Reference '{}' does not contain separator '//'",
                reference
            );
            log::error!("{}", &errstr);
            return Err(ImageError::new().with(TransportError(errstr)));
        }

        let ref_reference = tokens.get(1).unwrap();

        log::debug!("Parsing Reference '{}'", ref_reference);
        let result = parse(ref_reference);
        match result {
            Ok(r) => Ok(Box::new(r)),
            Err(e) => Err(ImageError::new().with(e)),
        }
    }

    fn cloned(&self) -> Box<dyn ImageTransport + Send + Sync> {
        Box::new(*self)
    }
}

#[derive(Debug)]
struct TransportError(String);

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Transport Error: {}", self.0)
    }
}

impl StdError for TransportError {}

impl From<TransportError> for ImageError {
    fn from(e: TransportError) -> Self {
        ImageError::new().with(e)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_reference() {
        struct ParseRefTC<'a> {
            input: &'a str,
            result: bool,
        }

        let transport = DockerTransport::new();
        let test_cases = vec![
            ParseRefTC {
                input: "docker://fedora",
                result: true,
            },
            ParseRefTC {
                input: "docker://",
                result: false,
            },
            ParseRefTC {
                input: "docker://docker://",
                result: false,
            },
            ParseRefTC {
                input: "",
                result: false,
            },
            ParseRefTC {
                input: "docker",
                result: false,
            },
            ParseRefTC {
                input: "docker://fedora/", // Invalid Reference
                result: false,
            },
        ];

        for tc in test_cases {
            let result = transport.parse_reference(tc.input);
            assert_eq!(result.is_ok(), tc.result);

            if result.is_ok() {
                assert_eq!(result.unwrap().transport().name(), "docker");
            }
        }
    }
}
