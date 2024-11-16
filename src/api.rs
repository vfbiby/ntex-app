use ntex::web::{self, types::{Json, State}, HttpResponse, Error};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;
use tracing::{info, error};

use crate::db;

#[web::get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}

#[web::get("/videos")]
async fn list_videos(db: State<DatabaseConnection>) -> Result<HttpResponse, Error> {
    info!("Listing all videos");
    match db::list_videos(&db).await {
        Ok(videos) => Ok(HttpResponse::Ok().json(&videos)),
        Err(err) => {
            error!("Database error: {}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVideoRequest {
    #[validate(length(min = 1, max = 100, message = "Title must be between 1 and 100 characters"))]
    title: String,
    #[validate(length(min = 1, message = "YouTube ID cannot be empty"))]
    youtube_id: String,
}

#[web::post("/videos")]
async fn create_video(
    db: State<DatabaseConnection>,
    payload: Json<CreateVideoRequest>,
) -> Result<HttpResponse, Error> {
    info!("Creating new video: {}", payload.title);
    
    if let Err(err) = payload.validate() {
        error!("Validation error: {}", err);
        let error_response = json!({
            "error": err.to_string()
        });
        return Ok(HttpResponse::BadRequest().json(&error_response));
    }

    match db::create_video(
        &db,
        payload.title.clone(),
        payload.youtube_id.clone(),
    ).await {
        Ok(video) => Ok(HttpResponse::Created().json(&video)),
        Err(err) => {
            error!("Database error: {}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{app::config_app, db::Model as Video};
    use ntex::http::StatusCode;
    use ntex::web::test::{self, TestRequest};
    use ntex::web::App;
    use serde_json::json;

    #[ntex::test]
    async fn test_create_video() {
        let db = crate::db::init_db().await;
        let app = test::init_service(
            App::new()
                .state(db)
                .configure(config_app)
        ).await;

        let payload = json!({
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

    #[ntex::test]
    async fn test_create_video_validation() {
        let db = crate::db::init_db().await;
        let app = test::init_service(
            App::new()
                .state(db)
                .configure(config_app)
        ).await;

        let payload = json!({
            "title": "",
            "youtube_id": ""
        });

        let req = TestRequest::post()
            .uri("/videos")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
