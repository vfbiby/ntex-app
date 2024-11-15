use ntex::http::StatusCode;
use ntex::web::test::TestRequest;

mod common;
use common::{assert_body, assert_header, assert_status};

#[ntex::test]
async fn test_hello_endpoint_returns_200() {
    assert_status(TestRequest::get().uri("/"), StatusCode::OK).await;
}

#[ntex::test]
async fn test_hello_endpoint_returns_plain_text() {
    assert_header(TestRequest::get().uri("/"), "content-type", "text/plain").await;
}

#[ntex::test]
async fn test_hello_endpoint_returns_hello_world() {
    assert_body(TestRequest::get().uri("/"), b"Hello world!").await;
}

#[ntex::test]
async fn test_not_found_returns_404() {
    assert_status(TestRequest::get().uri("/not-exists"), StatusCode::NOT_FOUND).await;
}
