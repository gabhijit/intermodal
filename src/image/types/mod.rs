#![allow(dead_code)]

//! Definitions of traits required for handling Images.
//!
//! # Reference:
//! [Types Implemented in Go](https://github.com/containers/image/blob/master/types/types.go)
//!
//! We are not going to define the types matching one to one above, but instead, the idea is to
//! have Interface definitions that would broadly achieve everything that the interfaces above
//! achieve.

use std::boxed::Box;

/// A Result of operations related to handling Images
pub type ImageResult<T> = Result<T, errors::ImageError>;

/// A trait that is to be implemented by All supported Image Transports
pub trait ImageTransport {
    /// Name of the Transport
    fn name(&self) -> String;

    /// Parse an input reference, that returns an ImageResult
    fn parse_reference<'s>(&self, reference: &'s str) -> ImageResult<Box<dyn ImageReference + 's>>;

    #[doc(hidden)]
    // We need to implement this for Transports because we are keeping a set of Transports in a
    // Hashmap, and then we'll have to return clone of the value in the HashMap. The additional
    // `Sync` and `Senc` requirements are because the HashMap is protected by a Mutex (being a
    // global variable).

    fn cloned(&self) -> Box<dyn ImageTransport + Send + Sync>;
    // fn validay_policy_config_scope<'a>(&self, scope: &'a str) -> ImageResult<()>;
}

// Required for handling the Boxed Trait Objects of ImageTransport type
impl Clone for Box<dyn ImageTransport + Send + Sync> {
    fn clone(&self) -> Self {
        self.cloned()
    }
}

/// A trait that should be implemented by All Image References
pub trait ImageReference {
    /// Returns the `ImageTransport` providing this Image Reference.
    fn transport(&self) -> Box<dyn ImageTransport + Send + Sync>;

    /// Returns the String within the transport that can be used to obtain the equivalent reference
    /// as the current reference.
    ///
    /// Thus `self.transport().parse_reference(self.string_within_reference())` will return a
    /// reference equivalent to the current reference.
    fn string_within_transport(&self) -> String;

    /// Returns an Image Source from the Reference provided or an Error.
    fn new_image_source(&self) -> ImageResult<Box<dyn ImageSource>>;
    // FIXME: implement following methods
    // fn docker_reference(&self) -> Box<dyn NamedRef>;

    // fn policy_configuration_identity(&self) -> String;

    // fn policy_configuration_namespaces(&self) -> Vec<String>;

    // fn new_image<T>(&self) -> T;
    // fn new_image_destination(&self) -> Result
}

/// A trait that should be implemented by All Image Sources.
///
/// An ImageSource is typically useful while copying the images.
pub trait ImageSource {
    /// Returns a Reference corresponding to this particular ImageSource.
    fn reference(&self) -> Box<dyn ImageReference>;

    // FIXME: implement following functions
    //
    // Owner of this should call `close` to free resources associated
    //fn close(&self) -> ImageResult<()>;

    // Returns the manifest and it's MIME type
    //fn get_manifest(&self, digest: &Digest) -> ImageResult<ImageManifest>;
}

/// A struct representing Image Manfest
pub struct ImageManifest {
    manifest: Vec<u8>,
    mime_type: &'static str,
}

pub mod errors;
