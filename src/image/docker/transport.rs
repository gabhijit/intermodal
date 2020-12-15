use std::boxed::Box;
use std::string::String;

use crate::image::types::{ImageReference, ImageResult, ImageTransport};

pub(in crate::image) fn get_docker_transport() -> (String, Box<dyn ImageTransport>) {
    (String::from("docker"), Box::new(DockerTransport {}))
}

pub(in crate::image) struct DockerTransport {}

impl ImageTransport for DockerTransport {
    fn name(&self) -> String {
        String::from("docker")
    }

    fn parse_reference<'a>(&self, reference: &'a str) -> ImageResult<Box<dyn ImageReference>> {
        Ok(Box::new(DockerReference {}))
    }
}

pub(in crate::image) struct DockerReference {}

impl ImageReference for DockerReference {
    fn transport(&self) -> Box<dyn ImageTransport> {
        Box::new(DockerTransport {})
    }
}
