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
/// Right now only docker transport is supported, when we support additional transports, we will
/// need to revisit the function to make sure that all transports can be properly obtained.
///
pub fn init_transports() {
    let (name, obj) = get_docker_transport();

    {
        let mut map = ALL_TRANSPORTS_MAP.lock().unwrap();
        map.insert(name, obj);
    }
}

pub fn parse_image_name(image_name: &str) -> ImageResult<Box<dyn ImageReference + '_>> {
    let tokens: Vec<&str> = image_name.split(':').collect();

    if tokens.len() != 2 {
        return Err(ImageError::ParseError);
    }

    {
        let transport_name = tokens.get(0).unwrap().to_string();
        let reference_part = tokens.get(1).unwrap();

        let map = ALL_TRANSPORTS_MAP.lock().unwrap();
        let transport = map.get(&transport_name).unwrap();

        transport.parse_reference(reference_part)
    }
}

pub fn transport_from_image_name(
    image_name: &str,
) -> Option<Box<dyn ImageTransport + Send + Sync>> {
    let tokens: Vec<&str> = image_name.split(':').collect();

    if tokens.len() != 2 {
        return None;
    }

    {
        let transport_name = tokens.get(0).unwrap().to_string();
        let map = ALL_TRANSPORTS_MAP.lock().unwrap();

        if map.contains_key(&transport_name) {
            let transport = map.get(&transport_name).unwrap();
            let cloned = (*transport).clone();
            Some(cloned)
        } else {
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
