use std::boxed::Box;
use std::string::String;

use crate::image::docker::reference::api::parse;

use crate::image::types::{ImageError, ImageReference, ImageResult, ImageTransport};

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
        let result = parse(reference);
        match result {
            Ok(r) => Ok(Box::new(r)),
            Err(_) => Err(ImageError::ReferenceError), // FIXME: May be give a detailed error later
        }
    }
}
