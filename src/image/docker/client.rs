#![allow(dead_code)]
// Docker client for Image registry
//

use hyper::{body::Body, client::connect::HttpConnector, Client as HyperClient, Uri};
use hyper_tls::HttpsConnector;

use crate::image::docker::reference::api::{parse, DEFAULT_DOCKER_DOMAIN};
use crate::image::types::ImageReference;

const DOCKER_REGISTRY_V2_HTTPS_URL: &str = "https://registry-1.docker.io";
const DOCKER_TAGS_PATH_FMT: &str = "/v2/{}/tags/list";
const DOCKER_MANIFESTS_PATH_FMT: &str = "/v2/{}/manifests/{}";
const DOCKER_BLOBS_PATH_FMT: &str = "/v2/{}/blobs/{}";

#[derive(Debug)]
pub(crate) struct DockerClient {
    pub(crate) https_client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    pub(crate) repo_url: Uri,
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

                let mut url = r.repo.domain.parse::<Uri>().unwrap();

                if url == DEFAULT_DOCKER_DOMAIN {
                    url = DOCKER_REGISTRY_V2_HTTPS_URL.parse::<Uri>().unwrap();
                }
                if url.scheme_str().is_none() || url.scheme_str() == Some("http") {
                    https_only = false;
                }

                let mut https_connector = HttpsConnector::new();
                https_connector.https_only(https_only);

                let https_client = HyperClient::builder().build(https_connector);
                Some(DockerClient {
                    https_client,
                    repo_url: url,
                })
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
        let image_ref = DockerTransport::new().parse_reference(image).unwrap();

        let client = DockerClient::new(image_ref);

        assert!(client.is_some());
    }

    #[test]
    fn test_new_client_failure() {}
}
