use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

use super::docker::transport::get_docker_transport;
use super::types::errors::ImageError;
use super::types::{ImageReference, ImageResult, ImageTransport};

lazy_static! {
    static ref ALL_TRANSPORTS_MAP: Mutex<HashMap<String, Box<dyn ImageTransport + Send + Sync>>> =
        Mutex::new(HashMap::new());
}

/// A function that initializes all supported transports
///
pub fn init_transports() {
    // Right now only docker transport is supported, when we support additional transports, we will
    // need to revisit the function to make sure that all transports can be properly obtained.
    let (name, obj) = get_docker_transport();

    {
        log::debug!("Registering '{}' Transport.", name);
        let mut map = ALL_TRANSPORTS_MAP.lock().unwrap();
        map.insert(name, obj);
    }
}

/// Parses the given input image_name and return a Result with success value as a Boxed
/// trait object implementing `ImageReference` trait.
pub fn parse_image_name<'a>(image_name: &'a str) -> ImageResult<Box<dyn ImageReference + 'a>> {
    let tokens: Vec<&str> = image_name.splitn(2, ':').collect();

    if tokens.len() != 2 {
        log::error!("Input Image name '{}' is invalid.", image_name);
        return Err(ImageError::new()); //  FIXME: Get a detailed info
    }

    {
        let transport_name = tokens.first().unwrap().to_string();
        let reference_part = tokens.get(1).unwrap();

        log::debug!(
            "Getting Transport object corresponding to Name:{}",
            transport_name
        );
        let map = ALL_TRANSPORTS_MAP.lock().unwrap();

        let transport = map.get(&transport_name).unwrap();

        transport.parse_reference(reference_part)
    }
}

/// Returns the Boxed Trait object implementing the `ImageTransport` trait or None.
pub fn transport_from_image_name(
    image_name: &str,
) -> Option<Box<dyn ImageTransport + Send + Sync>> {
    let tokens: Vec<&str> = image_name.split(':').collect();

    if tokens.len() != 2 {
        log::error!(
            "Input Image Name '{}' is not in valid <transport>:<path> format.",
            image_name
        );
        return None;
    }

    {
        let transport_name = tokens.first().unwrap().to_string();
        log::debug!(
            "Getting Transport object corresponding to {}",
            transport_name
        );

        let map = ALL_TRANSPORTS_MAP.lock().unwrap();
        if map.contains_key(&transport_name) {
            let transport = map.get(&transport_name).unwrap();
            let cloned = (*transport).clone();
            Some(cloned)
        } else {
            log::debug!(
                "Transport Object corresponding to Name {} Not found!",
                transport_name
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_transport_from_image_name_success() {
        init_transports();
        let result = transport_from_image_name("docker:fedora");

        assert!(result.is_some());
    }

    #[test]
    fn test_transport_from_image_name_failure() {
        init_transports();
        let result = transport_from_image_name("focker:fedora");

        assert!(result.is_none());
    }

    #[test]
    fn test_parse_image_name_succes() {
        init_transports();
        let result = parse_image_name("docker://fedora");

        assert!(result.is_ok());
        let reference = result.unwrap();
        assert_eq!(
            reference.string_within_transport(),
            "//docker.io/library/fedora:latest"
        );
    }

    #[test]
    fn test_parse_image_name_with_tag_success() {
        init_transports();
        let result = parse_image_name("docker://fedora:foo");

        assert!(result.is_ok());
        let reference = result.unwrap();
        assert_eq!(
            reference.string_within_transport(),
            "//docker.io/library/fedora:foo"
        );
    }

    #[test]
    fn test_parse_image_name_failure() {
        init_transports();
        let result = parse_image_name("docker");

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_image_name_failure_reference() {
        init_transports();
        let result = parse_image_name("docker:");

        assert!(result.is_err());
    }
}
