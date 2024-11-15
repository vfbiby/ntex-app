use ntex::web;
use ntex::web::HttpResponse;
use serde::Serialize;

#[web::get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}

#[derive(Serialize)]
struct Video {
    id: i32,
    title: String,
    youtube_id: String,
    created_at: String,
}

#[web::get("/videos")]
pub async fn list_videos() -> HttpResponse {
    let videos = Vec::<Video>::new();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(&videos)
}
