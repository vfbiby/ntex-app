use ntex::web::{self, HttpResponse};

// 实现handler
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}

// 添加main函数
#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
#[cfg(test)]
mod tests {
    use super::*;
    use ntex::util::Bytes;
    use ntex::web::{self, test};

    #[ntex::test]
    async fn test_hello_world() {
        // 创建测试应用
        let app = test::init_service(web::App::new().route("/", web::get().to(index))).await;

        // 创建测试请求
        let req = test::TestRequest::get().uri("/").to_request();

        // 发送请求并获取响应
        let resp = test::call_service(&app, req).await;

        // 验证状态码
        assert_eq!(resp.status(), 200);

        // 验证Content-Type (在读取body之前先获取headers)
        let headers = resp.headers();
        assert_eq!(
            headers.get("content-type").unwrap().to_str().unwrap(),
            "text/plain"
        );

        // 验证响应体
        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello world!"));
    }
}
