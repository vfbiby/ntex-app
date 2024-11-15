use ntex::http::{Request, StatusCode};
use ntex::util::Bytes;
use ntex::web::{test, test::TestRequest, Error, WebResponse};
use ntex::{web, Pipeline, Service};
use ntex_api::app::config_app;

pub async fn init_test_service(
) -> Pipeline<impl Service<Request, Response = WebResponse, Error = Error> + Sized> {
    test::init_service(web::App::new().configure(config_app)).await
}

// 辅助函数：验证响应状态码
pub async fn assert_status(req: TestRequest, expected_status: StatusCode) {
    let app = init_test_service().await;
    let resp = test::call_service(&app, req.to_request()).await;
    assert_eq!(resp.status(), expected_status);
}

// 辅助函数：验证响应头
pub async fn assert_header(req: TestRequest, header_name: &str, expected_value: &str) {
    let app = init_test_service().await;
    let resp = test::call_service(&app, req.to_request()).await;
    let headers = resp.headers();
    assert_eq!(
        headers.get(header_name).unwrap().to_str().unwrap(),
        expected_value
    );
}

// 辅助函数：验证响应体
pub async fn assert_body(req: TestRequest, expected_body: &[u8]) {
    let app = init_test_service().await;
    let resp = test::call_service(&app, req.to_request()).await;
    let body = test::read_body(resp).await;
    assert_eq!(body, Bytes::copy_from_slice(expected_body));
}
