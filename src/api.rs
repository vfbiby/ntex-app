use ntex::web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}
