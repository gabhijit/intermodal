#![allow(dead_code)]

//! Docker client for Image registry

use std::error::Error as StdError;
use std::fmt;

use hyper::http::{HeaderValue, StatusCode};
use hyper::{
    body::to_bytes, body::Body, client::HttpConnector, Client as HyperClient, Error as HyperError,
    Request, Uri,
};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use crate::image::docker::reference::api::DEFAULT_DOCKER_DOMAIN;
use crate::image::types::errors::ImageError;
use crate::image::types::ImageManifest;
use crate::oci::image::spec_v1::{MEDIA_TYPE_IMAGE_INDEX, MEDIA_TYPE_IMAGE_MANIFEST};

const DOCKER_REGISTRY_V2_HTTPS_URL: &str = "https://registry-1.docker.io";
const MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST: &str =
    "application/vnd.docker.distribution.manifest.v2+json";
const MEDIA_TYPE_DOCKER_V2_LIST: &str = "application/vnd.docker.distribution.manifest.list.v2+json";

#[derive(Debug)]
pub(super) struct ClientError(String);

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client Error: {}", self.0)
    }
}

impl StdError for ClientError {}

impl From<HyperError> for ClientError {
    fn from(e: HyperError) -> Self {
        ClientError(format!("Hyper Error: {}", e))
    }
}

impl From<ClientError> for ImageError {
    fn from(e: ClientError) -> Self {
        ImageError::new().with(e)
    }
}

/// Structure representing a Client for Docker Repository
#[derive(Debug, Clone)]
pub(super) struct DockerClient {
    https_client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    repo_url: Uri,
    // FIXME: This should be a Map of <scope, BearerToken>
    bearer_token: Option<BearerToken>,
}

impl DockerClient {
    /// Creates a New Docker Client from the Repository URL
    pub(super) fn new(repository: &str) -> Self {
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

    pub(super) async fn do_get_manifest(
        &mut self,
        path: &str,
        digest_or_tag: &str,
    ) -> Result<ImageManifest, ClientError> {
        if self.bearer_token.is_none() {
            self.get_bearer_token_for_path_scope(path, None).await?
        }

        if self.bearer_token.is_none() {
            let errstr = "Invalid Bearer Token Still!".to_string();
            log::error!("{}", &errstr);
            return Err(ClientError(errstr));
        }

        let manifest_url_path = format!("{}v2/{}/manifests/{}", self.repo_url, path, digest_or_tag);
        log::debug!("Getting Manifest: {}", manifest_url_path);

        let auth_header = format!("Bearer {}", self.bearer_token.as_ref().unwrap().token);
        let accept_header = vec![
            MEDIA_TYPE_IMAGE_INDEX,
            MEDIA_TYPE_IMAGE_MANIFEST,
            MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST,
            MEDIA_TYPE_DOCKER_V2_LIST,
        ]
        .join(", ");
        log::debug!("accept_header: {}", accept_header);
        let request = Request::builder()
            .method("GET")
            .uri(manifest_url_path)
            .header("Authorization", auth_header)
            .header("Accept", accept_header)
            .body(Body::from(""))
            .unwrap();

        log::debug!("Sending Request: {:#?}", request);
        let response = self.https_client.request(request).await?;
        let status = response.status();

        if status.is_success() {
            log::info!("Manifest Downloaded Successfully!");
            let mime_type = response
                .headers()
                .get("Content-Type")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let manifest = to_bytes(response).await?;
            Ok(ImageManifest {
                manifest: manifest.to_vec(),
                mime_type,
            })
        } else {
            let errstr = format!("Error in downloading Manifest: {}", status);
            log::error!("{}", &errstr);
            Err(ClientError(errstr))
        }
    }

    #[doc(hidden)]
    /// Performs API version check against the Docker Registry V2 API.
    ///
    /// Note: Only Docker Registry V2 is supported.
    async fn get_bearer_token_for_path_scope(
        &mut self,
        path: &str,
        scope: Option<&str>,
    ) -> Result<(), ClientError> {
        log::debug!(
            "Getting Bearer Token for Path: '{}', Scope: '{}'",
            path,
            scope.or(Some("")).unwrap()
        );

        let ping_url = format!("{}v2/", self.repo_url).parse::<Uri>().unwrap();

        log::trace!("Sending Request to {}", ping_url);
        let response = self.https_client.get(ping_url).await?;
        log::trace!("Received Response: {}", response.status());

        if response.status() == StatusCode::UNAUTHORIZED {
            log::trace!("Received 401. Checking For 'WWW-Authenticate' header.");
            if let Some(www_auth_header) = response.headers().get("WWW-Authenticate") {
                log::trace!(
                    "Got WWW-Authenticate Header: {}",
                    www_auth_header.to_str().unwrap()
                );

                let scope = if scope.is_none() {
                    log::trace!("Empty Scope, defaulting to 'pull'.");
                    "pull"
                } else {
                    scope.unwrap()
                };

                log::trace!("Sending Challenge Response.");
                let challenge_url = self
                    .prepare_auth_challenge_url(path, scope, www_auth_header)
                    .parse::<Uri>()
                    .unwrap();
                let auth_response = self.https_client.get(challenge_url).await?;
                let bearer_token = serde_json::from_slice::<'_, BearerToken>(
                    to_bytes(auth_response).await?.as_ref(),
                )
                .unwrap();

                self.bearer_token = Some(bearer_token);
                log::debug!("Bearer Token for Client Saved!");
                return Ok(());
            } else {
                let errstr = format!(
                    "No 'WWW-Authenticate' Header found with {}",
                    response.status()
                );
                log::error!("{}", &errstr);
                return Err(ClientError(errstr));
            }
        } else if response.status().is_success() {
            // unlikely path
            log::warn!("No Bearer Token for Client, but Ping response Success!. Bearer Token Not Obtained (and saved)!");
            return Ok(());
        } else {
            let errstr = format!("Error Getting Token: {}", response.status());
            log::error!("{}", &errstr);
            return Err(ClientError(errstr));
        }
    }

    // FIXME: This is hard-coded right now, when we can parse the header properly,
    // use parsed values.
    #[inline]
    fn prepare_auth_challenge_url(&self, path: &str, scope: &str, _: &HeaderValue) -> String {
        format!(
            "https://auth.docker.io/token?scope=repository:{}:{}&service=registry.docker.io",
            path, scope
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
struct BearerToken {
    token: String,
    access_token: String,
    issued_at: String,
    expires_in: u16,
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

        let result = client
            .get_bearer_token_for_path_scope("library/fedora", Some("pull"))
            .await;

        assert!(result.is_ok());
    }
}
