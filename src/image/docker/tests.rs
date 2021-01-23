use env_logger::Env;
use wiremock::{matchers::method, Mock, ResponseTemplate};

use super::{testdata::DOCKER_LIST_MANIFEST_BLOB, MEDIA_TYPE_DOCKER_V2_LIST};
use crate::image::{
    transports,
    types::{errors::ImageError, ImageReference},
};

fn init() {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("error"))
        .is_test(true)
        .try_init();
}

fn create_mock_reference<'a>(
    image_name: &'a str,
) -> Result<Box<dyn ImageReference + '_>, ImageError> {
    transports::parse_image_name(image_name)
}

#[tokio::test]
async fn test_new_image() {
    let server = wiremock::MockServer::start().await;
    let image_name = format!("docker://{}/library/fedora", server.address());

    let mock_ref = create_mock_reference(&image_name);
    assert!(mock_ref.is_ok(), "{:?}", mock_ref);

    let mock_image = mock_ref.unwrap().new_image();
    assert!(mock_image.is_ok());
}

#[tokio::test]
async fn test_get_manifest_list_success() {
    init();

    let mock_server = wiremock::MockServer::start().await;
    let image_name = format!("docker://{}/library/fedora", mock_server.address());

    let mock_response = ResponseTemplate::new(200).set_body_raw(
        DOCKER_LIST_MANIFEST_BLOB.as_bytes().to_owned(),
        MEDIA_TYPE_DOCKER_V2_LIST,
    );

    let mock_manifest = Mock::given(method("GET")).respond_with(mock_response);

    mock_server.register(mock_manifest).await;

    let mock_ref = create_mock_reference(&image_name);
    assert!(mock_ref.is_ok(), "{:?}", mock_ref);

    let mock_ref = mock_ref.unwrap();

    let ref_str = mock_ref.string_within_transport();
    let image = mock_ref.new_image();
    assert!(image.is_ok());

    let manifest = image.unwrap().manifest().await;

    assert!(manifest.is_ok(), "ref: {:?}, {:?}", ref_str, manifest);
}
