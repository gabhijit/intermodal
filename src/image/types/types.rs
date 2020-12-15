use super::errors::ImageResult;
use crate::oci::digest::Digest;

pub trait Reference {
    fn string(&self) -> String;
}

pub trait Named {
    fn name(&self) -> String;
}

pub trait Tagged {
    fn tag(&self) -> String;
}

pub trait NamedRef: Named + Reference {}

pub trait TaggedRef: Tagged + Reference {}

pub trait NamedTaggedRef: Named + Tagged + Reference {}

pub trait Digested {
    fn digest(&self) -> Digest;
}

pub trait ImageTransport {
    fn name(&self) -> String;

    fn parse_reference<'a>(&self, reference: &'a str) -> ImageResult<Box<dyn ImageReference>>;

    // fn validay_policy_config_scope<'a>(&self, scope: &'a str) -> ImageResult<()>;
}

pub trait ImageReference {
    fn transport(&self) -> Box<dyn ImageTransport>;

    // fn string_within_transport(&self) -> String;

    // fn docker_reference(&self) -> Box<dyn NamedRef>;

    // fn policy_configuration_identity(&self) -> String;

    // fn policy_configuration_namespaces(&self) -> Vec<String>;

    // FIXME: implement following methods
    // fn new_image<T>(&self) -> T;
    // fn new_image_source(&self) -> Result
    // fn new_image_destination(&self) -> Result
}
