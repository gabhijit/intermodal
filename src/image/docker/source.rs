//! Implementation of Docker specific ImageSource

use crate::image::types::{ImageReference, ImageSource};

use super::client::DockerClient;
use super::reference::types::DockerReference;

#[derive(Clone, Debug)]
pub(crate) struct DockerSource {
    pub(crate) reference: DockerReference,
    pub(crate) client: DockerClient,
}

impl ImageSource for DockerSource {
    fn reference(&self) -> Box<dyn ImageReference> {
        Box::new(self.reference.clone())
    }
}
