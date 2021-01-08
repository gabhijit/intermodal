//! Definitions of traits required for handling container Images.
//!
//! # Reference:
//! [Types Implemented in Go](https://github.com/containers/image/blob/master/types/types.go)
//!
//! We are not going to define the types matching one to one above, but instead, the idea is to
//! have Interface definitions that would broadly achieve everything that the interfaces above
//! achieve.

use std::boxed::Box;
use std::collections::HashMap;

use async_trait::async_trait;

use crate::oci::digest::Digest;

/// A Result of operations related to handling Images
pub type ImageResult<T> = Result<T, errors::ImageError>;

/// A trait that is to be implemented by All supported Image Transports
pub trait ImageTransport: std::fmt::Debug {
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
pub trait ImageReference: std::fmt::Debug {
    /// Returns the `ImageTransport` providing this Image Reference.
    fn transport(&self) -> Box<dyn ImageTransport + Send + Sync>;

    /// Returns the String within the transport that can be used to obtain the equivalent reference
    /// as the current reference.
    ///
    /// Thus `self.transport().parse_reference(self.string_within_reference())` will return a
    /// reference equivalent to the current reference.
    fn string_within_transport(&self) -> String;

    /// Returns an Image Source from the Reference provided or an Error.
    fn new_image_source(&self) -> ImageResult<Box<dyn ImageSource + Send + Sync>>;

    /// Returns the Image
    fn new_image(&self) -> ImageResult<Box<dyn Image>>;
    // FIXME: implement following methods
    // fn docker_reference(&self) -> Box<dyn NamedRef>;

    // fn policy_configuration_identity(&self) -> String;

    // fn policy_configuration_namespaces(&self) -> Vec<String>;

    // fn new_image_destination(&self) -> Result
}

/// A trait that should be implemented by All Image Sources.
///
/// An ImageSource is typically useful while copying the images.
#[async_trait]
pub trait ImageSource: std::fmt::Debug {
    /// Returns a Reference corresponding to this particular ImageSource.
    fn reference(&self) -> Box<dyn ImageReference>;

    /// Get the manifest using this `ImageSource`.
    ///
    /// If the passed `Digest` is None, it means - Get the manifest for the reference, this source
    /// points to. Usually it means getting the manifest for the 'digest' if present in the
    /// reference or the 'tag' (default if not present) for the reference. When we explicitly pass
    /// the Digest, we are interested in manifest corresponding to this specific digest (Which
    /// usually is the manifest for the 'Image' if the previous manifest was a 'list' or 'index'
    /// type.
    async fn get_manifest(&mut self, digest: Option<&Digest>) -> ImageResult<ImageManifest>;

    /// Get a blob for the image
    ///
    /// It is up to the caller to decide whether the requested blob is a 'config' or a 'layer'
    /// blob.
    async fn get_blob(&mut self, digest: &Digest) -> ImageResult<Vec<u8>>; // FIXME: We need some kind of Stream interface here

    // FIXME: implement following functions
    //
    // Owner of this should call `close` to free resources associated
    //fn close(&self) -> ImageResult<()>;
}

/// A trait that should be implemented by all Images.
///
/// This trait is an API for inspecting images. An image is basically represented by ImageSource
/// and instance Digest. This can be a manifest list or a single instance.
#[async_trait]
pub trait Image: std::fmt::Debug {
    /// Reference of the 'image source'.
    fn reference(&self) -> Box<dyn ImageReference>;

    /// Manifest of the image
    async fn manifest(&mut self) -> ImageResult<ImageManifest>;

    /// Configuration for the Image
    async fn config_blob(&mut self) -> ImageResult<Vec<u8>>;
}

/// A struct representing Image Manfest
#[derive(Debug, Clone)]
pub struct ImageManifest {
    pub manifest: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug)]
enum LayerCompression {
    PreserveOriginal,

    Compress,

    Decompress,
}

#[derive(Debug)]
enum LayerCrypto {
    PreserveOriginalCrypto,

    Encrypt,

    Decrypt,
}

#[derive(Debug)]
pub struct BlobInfo {
    pub digest: Digest,

    pub size: i64,

    urls: Vec<String>,

    annotations: HashMap<String, String>,

    media_type: String,

    // FIXME: Omitted Compression Algorithm.
    compression_op: LayerCompression,

    encryption_op: LayerCrypto,
}

pub mod errors;
