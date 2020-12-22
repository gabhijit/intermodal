#![allow(dead_code)]
// Docker client for Image registry
//

use hyper::{body::Body, client::connect::HttpConnector, Client as HyperClient, Uri};
use hyper_tls::HttpsConnector;

use crate::image::docker::reference::api::parse;
use crate::image::types::ImageReference;

const DOCKER_REGISTRY_HTTPS_URL: &str = "https://registry-1.docker.io";
const DOCKER_TAGS_PATH_FMT: &str = "/v2/{}/tags/list";
const DOCKER_MANIFESTS_PATH_FMT: &str = "/v2/{}/manifests/{}";
const DOCKER_BLOBS_PATH_FMT: &str = "/v2/{}/blobs/{}";

#[derive(Debug)]
struct DockerClient {
    https_client: HyperClient<HttpsConnector<HttpConnector>, Body>,
}

impl DockerClient {
    pub fn new(docker_ref: Box<dyn ImageReference>) -> Option<Self> {
        let sep = "//";
        let refstr = docker_ref.string_within_transport();
        let refpart = refstr.find(sep)?;

        let refpart = &refstr[(refpart + sep.len())..];
        let docker_ref = parse(refpart);

        match docker_ref {
            Ok(r) => {
                let mut https_only = true;

                let url = r.repo.domain.parse::<Uri>().unwrap();
                if url.scheme_str().is_none() || url.scheme_str() == Some("http") {
                    https_only = false;
                }

                let mut https_connector = HttpsConnector::new();
                https_connector.https_only(https_only);

                let https_client = HyperClient::builder().build(https_connector);
                Some(DockerClient { https_client })
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::image::docker::transport::DockerTransport;
    use crate::image::types::ImageTransport;

    #[test]
    fn test_new_client_success() {
        let image = "docker://fedora";
        let image_ref = DockerTransport::singleton().parse_reference(image).unwrap();

        let client = DockerClient::new(image_ref);

        assert!(client.is_some());
    }

    #[test]
    fn test_new_client_failure() {}
}
