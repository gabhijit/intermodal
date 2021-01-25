use env_logger::Env;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use super::{
    testdata::{DOCKER_IMAGE_MANIFEST_BLOB, DOCKER_LIST_MANIFEST_BLOB},
    MEDIA_TYPE_DOCKER_V2_LIST, MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST,
};
use crate::image::{
    transports,
    types::{errors::ImageError, ImageReference},
};

fn init() {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
        .is_test(true)
        .try_init();
}

async fn setup_mock_docker_api_server() -> MockServer {
    let mock_server = MockServer::start().await;

    let mock_ping = Mock::given(method("GET"))
        .and(path("/v2/"))
        .respond_with(ResponseTemplate::new(200));
    mock_server.register(mock_ping).await;

    let mock_list_response = ResponseTemplate::new(200).set_body_raw(
        DOCKER_LIST_MANIFEST_BLOB.as_bytes().to_owned(),
        MEDIA_TYPE_DOCKER_V2_LIST,
    );
    let mock_list_manifest = Mock::given(method("GET"))
        .and(path("/v2/library/fedora/manifests/latest"))
        .respond_with(mock_list_response);
    mock_server.register(mock_list_manifest).await;

    let mock_blob_response = ResponseTemplate::new(200).set_body_raw(
        DOCKER_IMAGE_MANIFEST_BLOB.as_bytes().to_owned(),
        MEDIA_TYPE_DOCKER_V2_SCHEMA2_MANIFEST,
    );
    let mock_blob_manifest = Mock::given(method("GET")).and(path("/v2/library/fedora/manifests/sha256:fdf235fa167d2aa5d820fba274ec1d2edeb0534bd32d28d602a19b31bad79b80")).respond_with(mock_blob_response);
    mock_server.register(mock_blob_manifest).await;

    mock_server
}

fn create_mock_reference<'a>(
    image_name: &'a str,
) -> Result<Box<dyn ImageReference + '_>, ImageError> {
    transports::parse_image_name(image_name)
}

#[tokio::test]
async fn test_new_image() {
    let server = setup_mock_docker_api_server().await;
    let image_name = format!("docker://{}/library/fedora", server.address());

    let mock_ref = create_mock_reference(&image_name);
    assert!(mock_ref.is_ok(), "{:?}", mock_ref);

    let mock_image = mock_ref.unwrap().new_image();
    assert!(mock_image.is_ok());
}

#[tokio::test]
async fn test_get_manifest_list_success() {
    init();
    let mock_server = setup_mock_docker_api_server().await;

    let image_name = format!("docker://{}/library/fedora", mock_server.address());

    let mock_ref = create_mock_reference(&image_name);
    assert!(mock_ref.is_ok(), "{:?}", mock_ref);

    let mock_ref = mock_ref.unwrap();

    let ref_str = mock_ref.string_within_transport();
    let image = mock_ref.new_image();
    assert!(image.is_ok());

    let manifest = image.unwrap().manifest().await;

    assert!(manifest.is_ok(), "ref: {:?}, {:?}", ref_str, manifest);
}

#[tokio::test]
async fn test_get_resolved_manifest() {
    init();
    let mock_server = setup_mock_docker_api_server().await;

    let image_name = format!("docker://{}/library/fedora", mock_server.address());

    let mock_ref = create_mock_reference(&image_name);
    assert!(mock_ref.is_ok(), "{:?}", mock_ref);

    let mock_ref = mock_ref.unwrap();

    let ref_str = mock_ref.string_within_transport();
    let image = mock_ref.new_image();
    assert!(image.is_ok());

    let manifest = image.unwrap().resolved_manifest().await;

    assert!(manifest.is_ok(), "ref: {:?}, {:?}", ref_str, manifest);
}
