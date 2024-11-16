use chrono::Utc;
use ntex::{
    web::{self, types::Json, HttpResponse},
};
use ntex::web::Responder;
use serde::{Deserialize, Serialize};

#[web::get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}

#[derive(Serialize, Deserialize)]
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

#[derive(Deserialize)]
pub struct CreateVideoRequest {
    title: String,
    youtube_id: String,
}

#[web::post("/videos")]
pub async fn create_video(payload: Json<CreateVideoRequest>) -> impl Responder {
    let video = Video {
        id: 1, // For now, hardcode id as 1
        title: payload.title.clone(),
        youtube_id: payload.youtube_id.clone(),
        created_at: Utc::now().to_rfc3339(),
    };

    HttpResponse::Created()
        .content_type("application/json")
        .json(&video)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::config_app;
    use ntex::http::StatusCode;
    use ntex::web::App;
    use ntex::web::test::{self, TestRequest};

    #[ntex::test]
    async fn test_create_video() {
        let app = test::init_service(App::new().configure(config_app)).await;

        let payload = serde_json::json!({
            "title": "Test Video",
            "youtube_id": "dQw4w9WgXcQ"
        });

        let req = TestRequest::post()
            .uri("/videos")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let body = test::read_body(resp).await;
        let video: Video = serde_json::from_slice(&body).unwrap();
        assert_eq!(video.title, "Test Video");
        assert_eq!(video.youtube_id, "dQw4w9WgXcQ");
        assert!(video.id > 0);
        assert!(!video.created_at.is_empty());
    }
}
