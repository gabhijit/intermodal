//! Docker client for Image registry

use core::convert::{Into, TryFrom};
use std::boxed::Box;
use std::error::Error as StdError;
use std::fmt;
use std::sync::RwLock;

use bytes::Bytes;
use chrono::{DateTime, Duration, Utc};
use futures::stream::Stream;
use futures_util::StreamExt;
use hyper::http::{
    header::{ACCEPT, AUTHORIZATION, LOCATION},
    Error as HttpError, HeaderMap, HeaderValue, Method as HttpMethod, StatusCode,
};
use hyper::{
    body::{to_bytes, Body},
    client::HttpConnector,
    Client as HyperClient, Error as HyperError, Request, Response, Uri,
};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

use crate::image::{
    docker::reference::api::DEFAULT_DOCKER_DOMAIN, manifest::DEFAULT_SUPPORTED_MANIFESTS,
    oci::digest::Digest, types::errors::ImageError, types::ImageManifest,
};
use crate::utils::image_blobs_cache_root;

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

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        ClientError(e.to_string())
    }
}

impl From<ClientError> for ImageError {
    fn from(e: ClientError) -> Self {
        ImageError::new().with(e)
    }
}

/// Structure representing a Client for Docker Repository
#[derive(Debug)]
pub(super) struct DockerClient {
    https_client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    repo_url: Uri,
    // FIXME: This should be a Map of <scope, BearerToken>
    bearer_token: RwLock<Option<BearerToken>>,
    auth_required: RwLock<bool>,
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

            let scheme: &str;
            if repo_url.port().is_some() {
                scheme = "http";
            } else {
                scheme = "https";
            };
            if repo_url.scheme().is_none() {
                repo_url = Uri::builder()
                    .scheme(scheme)
                    .authority(repository)
                    .path_and_query("/")
                    .build()
                    .unwrap();
            }
        }

        log::debug!("Getting DockerClient for '{}'.", repo_url);

        let mut https_only = false;
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
            bearer_token: RwLock::new(None),
            auth_required: RwLock::new(true),
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
            .body(Body::from(""))
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
                crate::log_err_return!(ClientError, "Error in Downloading Blob: {}", status);
            }
            if status.is_redirection() {
                if !response.headers().contains_key(LOCATION) {
                    let loc = LOCATION;

                    crate::log_err_return!(
                        ClientError,
                        "Redirect received but no {:?} Header.",
                        loc
                    );
                }
                let redirect_url = response.headers().get(LOCATION).unwrap().to_str().unwrap();
                log::trace!(
                    "Received Redirect to: {:?}. Trying to download.",
                    redirect_url
                );

                let request = Request::builder()
                    .method(method)
                    .uri::<&str>(redirect_url)
                    .body(Body::from(""))
                    .unwrap();

                log::trace!("Sending Request: {:#?}", request);
                let response = self.https_client.request(request).await?;
                let status = response.status();
                if status.is_success() {
                    return Ok(response);
                }
            }

            crate::log_err_return!(ClientError, "Error in Downloading: {}", status);
        }
    }

    /// Actually Get the manifest using the current client
    pub(super) async fn do_get_manifest(
        &self,
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

        if *self.auth_required.read().unwrap() {
            let auth_header = format!(
                "Bearer {}",
                self.bearer_token.read().unwrap().as_ref().unwrap().token
            );
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
        &self,
        path: &str,
        digest: &Digest,
    ) -> Result<Box<dyn Stream<Item = Bytes> + Unpin + Send + Sync>, ClientError> {
        let blob_url_path = format!("{}v2/{}/blobs/{}", self.repo_url, path, digest);
        log::debug!("Getting Blob: {}", blob_url_path);

        // This will get the bearer token and store it if required.
        self.get_bearer_token_for_path_scope(path, Some("pull"))
            .await?;

        let mut headers = HeaderMap::new();
        if *self.auth_required.read().unwrap() {
            let auth_header = format!(
                "Bearer {}",
                self.bearer_token.read().unwrap().as_ref().unwrap().token
            );
            headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        }

        let response = self
            .perform_http_request(blob_url_path, "GET", Some(&headers), true)
            .await?;

        let mut blobpath = std::env::temp_dir();
        blobpath.push("blobs");
        blobpath.push(digest.algorithm());
        std::fs::create_dir_all(&blobpath)?;

        blobpath.push(digest.hex_digest());
        let mut f = File::create(&blobpath).await?;

        let mut body = response.into_body();
        while let Some(data) = body.next().await {
            let data = data?;
            let _ = f.write(&data).await?;
        }
        f.flush().await?;

        log::trace!("***** Blobpath: {:?}", &blobpath);

        let f = File::open(&blobpath).await?;
        let result = digest
            .verify(&mut ReaderStream::new(f).map(|x| x.unwrap()))
            .await;
        if !result {
            crate::log_err_return!(
                ClientError,
                "Digest Verification failed for Digest: {}",
                digest
            );
        }

        log::trace!("Result of verify: {}", result);

        let mut cache_path = image_blobs_cache_root()?;
        cache_path.push(&digest.algorithm());
        std::fs::create_dir_all(&cache_path)?;
        cache_path.push(digest.hex_digest());
        std::fs::rename(&blobpath, &cache_path)?;

        let f = File::open(cache_path).await?;

        Ok(Box::new(ReaderStream::new(f).map(|x| x.unwrap())))
    }

    pub(super) async fn do_get_repo_tags(&self, path: &str) -> Result<Vec<String>, ClientError> {
        log::debug!("Getting Tags for the Repository: {}", path);
        let all_tags_url = format!("{}v2/{}/tags/list", self.repo_url, path);

        // This will get the bearer token and store it if required.
        self.get_bearer_token_for_path_scope(path, Some("pull"))
            .await?;

        let mut headers = HeaderMap::new();
        if *self.auth_required.read().unwrap() {
            let auth_header = format!(
                "Bearer {}",
                self.bearer_token.read().unwrap().as_ref().unwrap().token
            );
            headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        }

        let response = self
            .perform_http_request(all_tags_url, "GET", Some(&headers), true)
            .await?;

        let taginfo: TagInfo = serde_json::from_slice(&to_bytes(response).await?.to_vec())?;
        log::trace!("Received Tags: {:?}", taginfo);

        Ok(taginfo.tags)
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
        &self,
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
        if !*self.auth_required.read().unwrap() {
            return Ok(());
        }

        // We have a valid bearer token - No need to get it again.
        if self.is_valid_bearer_token() {
            return Ok(());
        }

        let response = self.ping_repository().await?;
        if response.status().is_success() {
            let mut auth_required = self.auth_required.write().unwrap();
            *auth_required = false;
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
                log::trace!("Auth Response: {}", std::str::from_utf8(&v).unwrap());
                let bearer_token = serde_json::from_slice::<'_, BearerToken>(&v).unwrap();

                log::trace!(
                    "Got Bearer Token: Issued At: {}, Expiring in: {}",
                    bearer_token.issued_at,
                    bearer_token.expires_in
                );

                {
                    let mut bt = self.bearer_token.write().unwrap();
                    *bt = Some(bearer_token);
                }

                log::debug!("Bearer Token for Client Saved!");
                return Ok(());
            } else {
                crate::log_err_return!(
                    ClientError,
                    "No 'WWW-Authenticate' Header found with {}",
                    response.status()
                );
            }
        } else if response.status().is_success() {
            // unlikely path
            log::warn!("No Bearer Token for Client, but Ping response Success!. Bearer Token Not Obtained (and saved)!");
            return Ok(());
        } else {
            crate::log_err_return!(ClientError, "Error Getting Token: {}", response.status());
        }
    }

    async fn ping_repository(&self) -> Result<Response<Body>, ClientError> {
        let ping_url = format!("{}v2/", self.repo_url).parse::<Uri>().unwrap();

        log::trace!("Sending Request to {}", ping_url);
        Ok(self.https_client.get(ping_url).await?)
    }

    fn is_valid_bearer_token(&self) -> bool {
        self.bearer_token.read().unwrap().is_some()
            && self
                .bearer_token
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .is_still_valid()
    }

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

    #[serde(default = "default_token")]
    access_token: String,

    #[serde(default = "issued_now")]
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
        let client = DockerClient::new(DOCKER_REGISTRY_V2_HTTPS_URL);

        let result = client
            .get_bearer_token_for_path_scope("library/fedora", Some("pull"))
            .await;

        assert!(result.is_ok());
    }
}
