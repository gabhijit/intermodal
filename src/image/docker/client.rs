//! Docker client for Image registry

use core::convert::{Into, TryFrom};
use std::error::Error as StdError;
use std::fmt;

use chrono::{DateTime, Duration, Utc};
use hyper::http::{
    header::{ACCEPT, AUTHORIZATION, LOCATION},
    Error as HttpError, HeaderMap, HeaderValue, Method as HttpMethod, StatusCode,
};
use hyper::{
    body::to_bytes, body::Body, client::HttpConnector, Client as HyperClient, Error as HyperError,
    Request, Response, Uri,
};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use crate::image::docker::reference::api::DEFAULT_DOCKER_DOMAIN;
use crate::image::manifest::DEFAULT_SUPPORTED_MANIFESTS;
use crate::image::types::errors::ImageError;
use crate::image::types::ImageManifest;
use crate::oci::digest::Digest;

const DOCKER_REGISTRY_V2_HTTPS_URL: &str = "https://registry-1.docker.io";

#[derive(Debug)]
pub(super) struct ClientError(String);

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client Error: {}", self.0)
    }
}

// Required to get tags list.
#[derive(Debug, Deserialize)]
struct TagInfo {
    name: String,
    tags: Vec<String>,
}

impl StdError for ClientError {}

impl From<HyperError> for ClientError {
    fn from(e: HyperError) -> Self {
        ClientError(format!("Hyper Error: {}", e))
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(e: serde_json::Error) -> Self {
        ClientError(format!("Serde JSON Error: {}", e))
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
    auth_required: bool,
}

impl DockerClient {
    /// Creates a New Docker Client from the Repository URL
    pub(super) fn new(repository: &str) -> Self {
        // We let panic if the Repo URL is not parseable

        let mut repo_url: Uri;
        if repository == DEFAULT_DOCKER_DOMAIN {
            repo_url = DOCKER_REGISTRY_V2_HTTPS_URL.parse::<Uri>().unwrap();
        } else {
            repo_url = repository.parse::<Uri>().unwrap();

            if repo_url.scheme().is_none() {
                repo_url = Uri::builder()
                    .scheme("https")
                    .authority(repository)
                    .path_and_query("/")
                    .build()
                    .unwrap();
            }
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
            auth_required: true,
        }
    }

    // FIXME: Handle taking 'body' as input
    /// Returns `Response` if it's a valid response or `ClientError`
    async fn perform_http_request<M, U>(
        &self,
        url: U,
        method: M,
        headers: Option<&HeaderMap>,
        handle_redirects: bool,
    ) -> Result<Response<Body>, ClientError>
    where
        HttpMethod: TryFrom<M>,
        <HttpMethod as TryFrom<M>>::Error: Into<HttpError>,
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<HttpError>,
        M: Copy,
    {
        let mut request = Request::builder()
            .method(method)
            .uri(url)
            .body(Body::from("")) // FIXME: Body
            .unwrap();

        if headers.is_some() {
            let headers = headers.unwrap();
            let req_headers = request.headers_mut();
            for (key, value) in headers {
                req_headers.insert(key, value.clone());
            }
        }

        log::trace!("Sending Request: {:#?}", request);
        let response = self.https_client.request(request).await?;
        let status = response.status();

        if status.is_success() {
            log::trace!("Downloaded Successfully!");
            Ok(response)
        } else {
            if !handle_redirects {
                let errstr = format!("Error in downloading Blob: {}", status);
                log::error!("{}", &errstr);
                return Err(ClientError(errstr));
            }
            if status.is_redirection() {
                if !response.headers().contains_key(LOCATION) {
                    let loc = LOCATION;
                    let errstr = format!("Redirect received but no {:?} Header.", loc);
                    log::error!("{}", &errstr);
                    return Err(ClientError(errstr));
                }
                let redirect_url = response.headers().get(LOCATION).unwrap().to_str().unwrap();
                log::trace!(
                    "Received Redirect to: {:?}. Trying to download.",
                    redirect_url
                );

                let request = Request::builder()
                    .method(method)
                    .uri::<&str>(redirect_url)
                    .body(Body::from("")) // FIXME: Body
                    .unwrap();

                log::trace!("Sending Request: {:#?}", request);
                let response = self.https_client.request(request).await?;
                let status = response.status();
                if status.is_success() {
                    return Ok(response);
                }
            }

            let errstr = format!("Error in downloading : {}", status);
            log::error!("{}", &errstr);
            Err(ClientError(errstr))
        }
    }

    /// Actually Get the manifest using the current client
    pub(super) async fn do_get_manifest(
        &mut self,
        path: &str,
        digest_or_tag: &str,
    ) -> Result<ImageManifest, ClientError> {
        let manifest_url = format!("{}v2/{}/manifests/{}", self.repo_url, path, digest_or_tag);
        log::debug!("Getting Manifest: {}", manifest_url);

        let mut headers = HeaderMap::new();

        let accept_header = DEFAULT_SUPPORTED_MANIFESTS.join(", ");
        headers.insert(ACCEPT, accept_header.parse().unwrap());

        // This will get the bearer token and store it.
        self.get_bearer_token_for_path_scope(path, Some("pull"))
            .await?;

        if self.auth_required {
            let auth_header = format!("Bearer {}", self.bearer_token.as_ref().unwrap().token);
            headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        }

        let response = self
            .perform_http_request(manifest_url, "GET", Some(&headers), true)
            .await?;
        let mime_type = response
            .headers()
            .get("Content-Type")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Ok(ImageManifest {
            manifest: to_bytes(response).await?.to_vec(),
            mime_type,
        })
    }

    pub(super) async fn do_get_blob(
        &mut self,
        path: &str,
        digest: &Digest,
    ) -> Result<Vec<u8>, ClientError> {
        let blob_url_path = format!("{}v2/{}/blobs/{}", self.repo_url, path, digest);
        log::debug!("Getting Blob: {}", blob_url_path);

        // This will get the bearer token and store it if required.
        self.get_bearer_token_for_path_scope(path, Some("pull"))
            .await?;

        let mut headers = HeaderMap::new();
        if self.auth_required {
            let auth_header = format!("Bearer {}", self.bearer_token.as_ref().unwrap().token);
            headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        }

        let response = self
            .perform_http_request(blob_url_path, "GET", Some(&headers), true)
            .await?;

        Ok(to_bytes(response).await?.to_vec())
    }

    // FIXME: This should take a mut self so that we can update Bearer Token if required.
    pub(super) async fn do_get_repo_tags(&self, path: &str) -> Result<Vec<String>, ClientError> {
        log::debug!("Getting Tags for the Repository: {}", path);
        let all_tags_url = format!("{}v2/{}/tags/list", self.repo_url, path);

        let mut headers = HeaderMap::new();
        if self.auth_required {
            let auth_header = format!("Bearer {}", self.bearer_token.as_ref().unwrap().token);
            headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        }

        let response = self
            .perform_http_request(all_tags_url, "GET", Some(&headers), true)
            .await?;

        let taginfo: TagInfo = serde_json::from_slice(&to_bytes(response).await?.to_vec())?;
        log::trace!("Received Tags: {:?}", taginfo);

        Ok(taginfo.tags.clone())
    }

    #[doc(hidden)]
    /// Performs API version check against the Docker Registry V2 API.
    ///
    /// Once the bearer token is obtained, it is cached at the client, so that we do not have to
    /// get one for every API use.
    ///
    /// Note: Only Docker Registry V2 is supported.
    ///
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

        // If we have already determined, no auth is required, no bearer token is needed to be
        // downloaded.
        if !self.auth_required {
            return Ok(());
        }

        // We have a valid bearer token - No need to get it again.
        if self.is_valid_bearer_token() {
            return Ok(());
        }

        let response = self.ping_repository().await?;
        if response.status().is_success() {
            self.auth_required = false;
            return Ok(());
        }

        // Got a 401 - We need to get the bearer token
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
                let v = to_bytes(auth_response).await?.to_vec();
                log::trace!("Auth Response: {}", String::from_utf8(v.clone()).unwrap());
                let bearer_token = serde_json::from_slice::<'_, BearerToken>(&v).unwrap();

                log::trace!(
                    "Got Bearer Token: Issued At: {}, Expiring in: {}",
                    bearer_token.issued_at,
                    bearer_token.expires_in
                );
                let _ = self.bearer_token.take(); // Get into a variable to drop it
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

    async fn ping_repository(&self) -> Result<Response<Body>, ClientError> {
        let ping_url = format!("{}v2/", self.repo_url).parse::<Uri>().unwrap();

        log::trace!("Sending Request to {}", ping_url);
        Ok(self.https_client.get(ping_url).await?)
    }

    fn is_valid_bearer_token(&self) -> bool {
        self.bearer_token.is_some() && self.bearer_token.as_ref().unwrap().is_still_valid()
    }

    // FIXME: This is hard-coded right now, when we can parse the header properly,
    // use parsed values.
    #[inline]
    fn prepare_auth_challenge_url(
        &self,
        path: &str,
        scope: &str,
        auth_header: &HeaderValue,
    ) -> String {
        log::trace!("{:?}", auth_header);
        let mut realm: Option<&str> = None;
        let mut service: Option<&str> = None;
        let header_vals: Vec<&str> = auth_header.to_str().unwrap().split_whitespace().collect();
        let auth_type = header_vals.get(0).unwrap();
        let auth_realm = header_vals.get(1).unwrap();
        log::trace!("auth_type: {}, auth_realm: {}", auth_type, auth_realm);
        let _ = auth_realm.split(',').for_each(|v| {
            let toks: Vec<&str> = v.split('=').collect();
            if let Some(first) = toks.get(0) {
                if *first == "realm" {
                    realm = Some(toks.get(1).unwrap().trim_matches('"'));
                }
                if *first == "service" {
                    service = Some(toks.get(1).unwrap().trim_matches('"'));
                }
            }
        });
        if realm.is_none() || service.is_none() {
            panic!("For now!");
        }

        format!(
            "{}?scope=repository:{}:{}&service={}",
            realm.unwrap(),
            path,
            scope,
            service.unwrap()
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
struct BearerToken {
    token: String,

    #[serde(default = "default_token")] // FIXME:
    access_token: String,

    #[serde(default = "issued_now")] // FIXME:
    issued_at: String,

    #[serde(default = "expires_in_60")]
    expires_in: u16,
}

fn default_token() -> String {
    "".to_string()
}

fn issued_now() -> String {
    Utc::now().to_rfc3339()
}

fn expires_in_60() -> u16 {
    60
}

impl BearerToken {
    fn is_still_valid(&self) -> bool {
        // If docker suddenly stops sending RFC 3339 compliant timestamps, let it panic!
        let expires_at = DateTime::parse_from_rfc3339(&self.issued_at)
            .unwrap()
            .checked_add_signed(Duration::seconds(self.expires_in.into()))
            .unwrap();
        let now = Utc::now();
        log::trace!("Expires At: {}, Now: {}", expires_at, now);
        expires_at > now
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::Utc;

    #[test]
    fn test_new_client_success() {
        let repo_url = "https://registry-1.docker.io/";
        let client = DockerClient::new(repo_url);

        assert_eq!(client.repo_url, repo_url);
    }

    #[test]
    fn test_bearer_token_valid() {
        let b = BearerToken {
            token: "some random token".to_string(),
            access_token: "some random access token".to_string(),
            issued_at: Utc::now().to_rfc3339(),
            expires_in: 2,
        };

        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(b.is_still_valid());
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert!(!b.is_still_valid());
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
