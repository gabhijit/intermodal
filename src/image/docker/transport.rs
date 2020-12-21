#![allow(clippy::redundant_allocation)]
use std::boxed::Box;
use std::string::String;

use crate::image::docker::reference::api::parse;

use crate::image::types::errors::ImageError;
use crate::image::types::{ImageReference, ImageResult, ImageTransport};

pub(crate) static DOCKER_TRANSPORT_NAME: &str = "docker";
pub(crate) static DOCKER_TRANSPORT: DockerTransport = DockerTransport {
    name: DOCKER_TRANSPORT_NAME,
};

pub(in crate::image) fn get_docker_transport<'s>() -> (String, Box<&'s (dyn ImageTransport)>) {
    (
        String::from(DOCKER_TRANSPORT_NAME),
        Box::new(&DOCKER_TRANSPORT),
    )
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct DockerTransport<'a> {
    pub(crate) name: &'a str,
}

impl<'a> DockerTransport<'a> {
    pub(crate) fn singleton() -> &'a Self {
        &DOCKER_TRANSPORT
    }
}

impl<'a> ImageTransport for DockerTransport<'a> {
    fn name(&self) -> String {
        String::from(self.name)
    }

    fn parse_reference<'s>(&self, reference: &'s str) -> ImageResult<Box<dyn ImageReference + 's>> {
        let dslash = reference.find("://");

        if dslash.is_none() {
            return Err(ImageError::ReferenceError);
        }

        let tokens: Vec<&str> = reference.split("://").collect();

        if tokens.len() != 2 {
            return Err(ImageError::ReferenceError);
        }

        let _ref_type = tokens.get(0).unwrap();
        let ref_reference = tokens.get(1).unwrap();

        let result = parse(ref_reference);
        match result {
            Ok(r) => Ok(Box::new(r)),
            Err(_) => Err(ImageError::ReferenceError), // FIXME: May be give a detailed error later
        }
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

        let transport = DockerTransport::singleton();
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
