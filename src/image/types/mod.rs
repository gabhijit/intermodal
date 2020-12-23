use std::boxed::Box;

pub type ImageResult<T> = Result<T, errors::ImageError>;

pub trait ImageTransport {
    fn name(&self) -> String;

    fn parse_reference<'s>(&self, reference: &'s str) -> ImageResult<Box<dyn ImageReference + 's>>;

    // fn validay_policy_config_scope<'a>(&self, scope: &'a str) -> ImageResult<()>;
}

pub trait ImageReference {
    #![allow(clippy::redundant_allocation)]
    fn transport(&self) -> Box<dyn ImageTransport + Send + Sync>;

    fn string_within_transport(&self) -> String;

    // fn docker_reference(&self) -> Box<dyn NamedRef>;

    // fn policy_configuration_identity(&self) -> String;

    // fn policy_configuration_namespaces(&self) -> Vec<String>;

    // FIXME: implement following methods
    // fn new_image<T>(&self) -> T;
    // fn new_image_source(&self) -> Result
    // fn new_image_destination(&self) -> Result
}

impl Clone for Box<dyn ImageTransport + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone()
    }
}

pub mod errors;
