use super::errors::ImageResult;

pub trait ImageTransport {
    fn name(&self) -> String;

    fn parse_reference<'s>(&self, reference: &'s str) -> ImageResult<Box<dyn ImageReference + 's>>;

    // fn validay_policy_config_scope<'a>(&self, scope: &'a str) -> ImageResult<()>;
}

pub trait ImageReference {
    #![allow(clippy::redundant_allocation)]
    fn transport<'it>(&'it self) -> Box<&(dyn ImageTransport + 'it)>;

    // fn string_within_transport(&self) -> String;

    // fn docker_reference(&self) -> Box<dyn NamedRef>;

    // fn policy_configuration_identity(&self) -> String;

    // fn policy_configuration_namespaces(&self) -> Vec<String>;

    // FIXME: implement following methods
    // fn new_image<T>(&self) -> T;
    // fn new_image_source(&self) -> Result
    // fn new_image_destination(&self) -> Result
}
