use ntex::{
    web::{self, types::{Json, State}, HttpResponse, Responder},
};
use serde::Deserialize;
use sea_orm::DatabaseConnection;

use crate::db;

#[web::get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}

#[web::get("/videos")]
pub async fn list_videos(db: State<DatabaseConnection>) -> HttpResponse {
    match db::list_videos(&db).await {
        Ok(videos) => HttpResponse::Ok().json(&videos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
pub struct CreateVideoRequest {
    title: String,
    youtube_id: String,
}

#[web::post("/videos")]
pub async fn create_video(
    db: State<DatabaseConnection>,
    payload: Json<CreateVideoRequest>
) -> impl Responder {
    match db::create_video(
        &db,
        payload.title.clone(),
        payload.youtube_id.clone(),
    ).await {
        Ok(video) => HttpResponse::Created().json(&video),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app::config_app, db::Model as Video};
    use ntex::http::StatusCode;
    use ntex::web::App;
    use ntex::web::test::{self, TestRequest};

    #[ntex::test]
    async fn test_create_video() {
        let db = crate::db::init_db().await;
        let app = test::init_service(
            App::new()
                .state(db)
                .configure(config_app)
        ).await;

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
