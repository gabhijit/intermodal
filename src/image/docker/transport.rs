#![allow(clippy::redundant_allocation)]
use std::boxed::Box;
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
            return Err(ImageError::ReferenceError);
        }

        let tokens: Vec<&str> = reference.split("//").collect();

        if tokens.len() != 2 {
            return Err(ImageError::ReferenceError);
        }

        let ref_reference = tokens.get(1).unwrap();

        let result = parse(ref_reference);
        match result {
            Ok(r) => Ok(Box::new(r)),
            Err(_) => Err(ImageError::ReferenceError), // FIXME: May be give a detailed error later
        }
    }

    fn cloned(&self) -> Box<dyn ImageTransport + Send + Sync> {
        Box::new(*self)
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
