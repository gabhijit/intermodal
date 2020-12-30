#![allow(dead_code)]

//! Docker client for Image registry

use hyper::http::{HeaderValue, StatusCode};
use hyper::{
    body::to_bytes, body::Body, client::HttpConnector, Client as HyperClient, Error as HyperError,
    Uri,
};
use hyper_tls::HttpsConnector;

use crate::image::docker::reference::api::DEFAULT_DOCKER_DOMAIN;

const DOCKER_REGISTRY_V2_HTTPS_URL: &str = "https://registry-1.docker.io";
const DOCKER_TAGS_PATH_FMT: &str = "/v2/{}/tags/list";
const DOCKER_MANIFESTS_PATH_FMT: &str = "/v2/{}/manifests/{}";
const DOCKER_BLOBS_PATH_FMT: &str = "/v2/{}/blobs/{}";

/// Structure representing a Client for Docker Repository
#[derive(Debug, Clone)]
pub(super) struct DockerClient {
    https_client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    repo_url: Uri,
    bearer_token: Option<BearerToken>,
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
            bearer_token: None,
        }
    }

    #[doc(hidden)]
    /// Performs API version check against the Docker Registry V2 API.
    ///
    /// Note: Only Docker Registry V2 is supported.
    pub(super) async fn get_bearer_token_for_path(&mut self, path: &str) -> Result<(), HyperError> {
        let ping_url = format!("{}v2/", self.repo_url).parse::<Uri>().unwrap();

        log::debug!("Sending Request to {}", ping_url);

        let response = self.https_client.get(ping_url).await.unwrap();

        log::debug!("Received Response: {}", response.status());

        if response.status() == StatusCode::UNAUTHORIZED {
            log::debug!("Received 401. Checking For 'WWW-Authenticate' header.");
            if let Some(www_auth_header) = response.headers().get("WWW-Authenticate") {
                log::debug!(
                    "Got WWW-Authenticate Header: {}",
                    www_auth_header.to_str().unwrap()
                );
                let challenge_url = self
                    .prepare_auth_challenge_url(path, www_auth_header)
                    .parse::<Uri>()
                    .unwrap();
                log::debug!("{}", challenge_url);
                let auth_response = self.https_client.get(challenge_url).await?;
                log::debug!("{}", auth_response.status());

                let body = to_bytes(auth_response).await?;
                log::debug!("{:#?}", std::str::from_utf8(&body).unwrap());
            }
        }
        Ok(())
    }

    pub(super) async fn do_get_manifest(
        &mut self,
        path: &str,
        digest_or_tag: &str,
    ) -> Result<(), HyperError> {
        let manifest_path = format!("{}{}/{}", self.repo_url, path, digest_or_tag);
        log::debug!("{}", manifest_path);

        if self.bearer_token.is_none() {
            self.get_bearer_token_for_path(path).await?
        }
        Ok(())
    }

    // FIXME: This is hard-coded right now, when we can parse the header properly,
    // use parsed values.
    fn prepare_auth_challenge_url(&self, path: &str, _: &HeaderValue) -> String {
        format!(
            "https://auth.docker.io/token?repository:{}:pull&service=registry.docker.io",
            path
        )
    }
}

#[derive(Debug, Clone)]
struct BearerToken;

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

        let result = client.get_bearer_token_for_path("").await;

        assert!(result.is_ok());
    }
}
