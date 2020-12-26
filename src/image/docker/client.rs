#![allow(dead_code)]

//! Docker client for Image registry

use hyper::{body::Body, client::connect::HttpConnector, Client as HyperClient, Uri};
use hyper_tls::HttpsConnector;
use log;

const DOCKER_REGISTRY_V2_HTTPS_URL: &str = "https://registry-1.docker.io";
const DOCKER_TAGS_PATH_FMT: &str = "/v2/{}/tags/list";
const DOCKER_MANIFESTS_PATH_FMT: &str = "/v2/{}/manifests/{}";
const DOCKER_BLOBS_PATH_FMT: &str = "/v2/{}/blobs/{}";

/// Structure representing a Client for Docker Repository
#[derive(Debug, Clone)]
pub(crate) struct DockerClient {
    pub(crate) https_client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    pub(crate) repo_url: Uri,
}

impl DockerClient {
    /// Creates a New Docker Client from the Repository URL
    pub fn new(repo_url: &str) -> Self {
        // We let panic if the Repo URL is not parseable

        log::debug!("Getting DockerClient for '{}'.", repo_url);
        let repo_url = repo_url.parse::<Uri>().unwrap();

        let mut https_only = true;
        if repo_url.scheme_str().is_none() || repo_url.scheme_str() == Some("http") {
            https_only = false;
            log::warn!("Using Non HTTPS scheme for the URL.");
        }

        let mut https_connector = HttpsConnector::new();
        https_connector.https_only(https_only);

        let https_client = HyperClient::builder().build(https_connector);

        DockerClient {
            https_client,
            repo_url,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_client_success() {
        let repo_url = "https://registry-1.docker.io/";
        let client = DockerClient::new(repo_url);

        assert_eq!(client.repo_url, repo_url);
    }
}
