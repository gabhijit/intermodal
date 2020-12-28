#![allow(dead_code)]

//! Docker client for Image registry

use hyper::http::StatusCode;
use hyper::{body::Body, client::HttpConnector, Client as HyperClient, Error as HyperError, Uri};
use hyper_tls::HttpsConnector;

use crate::image::docker::reference::api::DEFAULT_DOCKER_DOMAIN;

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
    pub fn new(repository: &str) -> Self {
        // We let panic if the Repo URL is not parseable

        let repo_url: Uri;
        if repository == DEFAULT_DOCKER_DOMAIN {
            repo_url = DOCKER_REGISTRY_V2_HTTPS_URL.parse::<Uri>().unwrap();
        } else {
            repo_url = repository.parse::<Uri>().unwrap();
        }

        log::debug!("Getting DockerClient for '{}'.", repo_url);

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

    #[doc(hidden)]
    /// Performs API version check against the Docker Registry V2 API.
    ///
    /// Note: Only Docker Registry V2 is supported.
    async fn api_version_check(&mut self) -> Result<(), HyperError> {
        let ping_url = format!("{}/v2/", self.repo_url).parse::<Uri>().unwrap();

        log::debug!("Sending Request to {}", ping_url);

        let response = self.https_client.get(ping_url).await.unwrap();

        if response.status() == StatusCode::UNAUTHORIZED {
            log::debug!("Received 401. Checking For 'WWW-Authenticate' header.");
            if let Some(www_auth_header) = response.headers().get("WWW-Authenticate") {
                log::debug!(
                    "Got WWW-Authenticate Header: {}",
                    www_auth_header.to_str().unwrap()
                );
            }
        }
        Ok(())
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

    #[tokio::test]
    async fn test_api_version_check() {
        let mut client = DockerClient::new(DOCKER_REGISTRY_V2_HTTPS_URL);

        let result = client.api_version_check().await;

        assert!(result.is_ok());
    }
}
