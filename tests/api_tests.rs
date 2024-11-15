use crate::common::init_test_service;
use ntex::util::Bytes;
use ntex::web::{test, test::TestRequest};

mod common;

#[ntex::test]
async fn test_hello_world() {
    let app = init_test_service().await;

    let req = TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let headers = resp.headers();
    assert_eq!(
        headers.get("content-type").unwrap().to_str().unwrap(),
        "text/plain"
    );

    let body = test::read_body(resp).await;
    assert_eq!(body, Bytes::from_static(b"Hello world!"));
}

#[ntex::test]
async fn test_not_found() {
    let app = init_test_service().await;

    let req = TestRequest::get().uri("/not-exists").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}
