use std::collections::HashMap;

use super::docker::transport::get_docker_transport;
use super::types::ImageTransport;

pub fn init_transports() -> HashMap<String, Box<dyn ImageTransport>> {
    let all_transports_map: HashMap<String, Box<dyn ImageTransport>> = HashMap::new();

    let (_name, _obj) = get_docker_transport();

    all_transports_map
}
