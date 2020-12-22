use std::collections::HashMap;

use super::docker::transport::get_docker_transport;
use super::types::ImageTransport;

pub fn init_transports() -> HashMap<String, Box<&'static (dyn ImageTransport)>> {
    let mut all_transports_map: HashMap<String, Box<&'static (dyn ImageTransport)>> =
        HashMap::new();

    let (name, obj) = get_docker_transport();

    all_transports_map.insert(name, obj);

    all_transports_map
}
