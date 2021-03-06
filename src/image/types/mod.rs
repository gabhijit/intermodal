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
use serde::Serialize;
use tokio::io::AsyncRead;

use crate::image::{
    docker::reference::types::DockerImageReference, oci::digest::Digest,
    oci::spec_v1::Image as OCIv1Image,
};

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
    // `Sync` and `Send` requirements are because the HashMap is protected by a Mutex (being a
    // global variable).

    fn cloned(&self) -> Box<dyn ImageTransport + Send + Sync>;
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
    fn new_image(&self) -> ImageResult<Box<dyn Image + Send + Sync>>;

    /// Returns the DockerReference corresponding to this ImageReference
    fn docker_reference(&self) -> Option<Box<dyn DockerImageReference>> {
        None
    }

    // FIXME: implement following methods
    // fn policy_configuration_identity(&self) -> String;

    // fn policy_configuration_namespaces(&self) -> Vec<String>;

    // fn new_image_destination(&self) -> Result
}

/// A trait that should be implemented by All Image Sources.
///
/// An ImageSource is an ImageReference and a client. The 'client' for the image source handles
/// 'transport' specific details. Thus we'll have an ImageSource for every soupported transport.
/// Right now we are supporting only 'docker' (Repo V2) and 'oci' (local FS - TODO).
///
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
    /// type.)
    async fn get_manifest(&mut self, digest: Option<&Digest>) -> ImageResult<ImageManifest>;

    /// Get a blob for the image
    ///
    /// It is up to the caller to decide whether the requested blob is a 'config' or a 'layer'
    /// blob.
    async fn get_blob(
        &self,
        digest: &Digest,
    ) -> ImageResult<Box<dyn AsyncRead + Unpin + Send + Sync>>;

    /// Get all tags for this Image source
    ///
    /// Get's all tags corresponding to this Image Source. Note: Right now this makes sense only
    /// for the 'docker' Image sources, for other image sources, simply return an Empty List.
    async fn get_repo_tags(&self) -> ImageResult<Vec<String>>;
}

/// A trait that should be implemented by all Images.
///
/// This trait is an API for inspecting images. An image is basically represented by ImageSource
/// and instance Digest. This can be a manifest list or a single instance.
#[async_trait]
pub trait Image: std::fmt::Debug {
    /// Underlying 'image source'
    fn source_ref(&self) -> &(dyn ImageSource + Send + Sync);

    /// Reference of the 'image source'.
    fn reference(&self) -> Box<dyn ImageReference>;

    /// Returns the manifest for the image.
    ///
    /// This manifest returns the 'manifest' for the source corresponding to `reference()`.
    /// The manifest may be of 'list' or 'index' type (if the reference is a tag) or appropriate
    /// media type (if the reference is a digest).
    ///
    /// Usually, if the reference is a user requested, it's likely that the manifest corresponds to
    /// a user requested tag or default tag 'latest'.
    ///
    async fn manifest(&mut self) -> ImageResult<ImageManifest>;

    /// Returns the 'Resolved' manifest for the image.
    ///
    /// Manifest for the `reference()` is resolved to current OS and Architecture and is returned.
    /// This manifest will be used to get other image details like config and layer blobs.
    ///
    async fn resolved_manifest(&mut self) -> ImageResult<ImageManifest>;

    /// Returns the raw config blob for the Image
    async fn config_blob(&mut self) -> ImageResult<Vec<u8>>;

    /// Returns the Image in OCI Format.
    async fn oci_config(&mut self) -> ImageResult<OCIv1Image>;

    /// Returns inspect output friendly structure.
    async fn inspect(&mut self) -> ImageResult<ImageInspect>;
}

/// A struct representing Image Manfest
#[derive(Debug, Clone)]
pub struct ImageManifest {
    pub manifest: Vec<u8>,
    pub mime_type: String,
}

/// A struct representing Inspect output (Something like 'docker inspect', 'skopeo inspect')
#[derive(Debug, Serialize)]
pub struct ImageInspect {
    #[serde(rename = "Created")]
    pub created: String,

    #[serde(rename = "DockerVersion")]
    pub docker_version: String,

    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "Architecture")]
    pub architecture: String,

    #[serde(rename = "Os")]
    pub os: String,

    #[serde(rename = "Layers")]
    pub layers: Vec<String>,

    #[serde(rename = "Env")]
    pub env: Vec<String>,
}

pub mod errors;
