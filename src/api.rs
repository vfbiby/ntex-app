use ntex::web;
use ntex::web::HttpResponse;

#[web::get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}
