use ntex::web::{self, types::{Json, Path, Query, State}, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use validator::Validate;
use tracing::info;
use chrono::{DateTime, Utc};

use crate::db::{self, VideoQuery};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateVideoRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 11, max = 11))]
    pub youtube_id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateVideoRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 11, max = 11))]
    pub youtube_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResponse {
    pub id: i32,
    pub title: String,
    pub youtube_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedVideoResponse {
    pub videos: Vec<VideoResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

#[web::get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome to Video API!")
}

#[web::post("/videos")]
pub async fn create_video(
    req: Json<CreateVideoRequest>,
    data: State<DatabaseConnection>,
) -> impl Responder {
    info!("Creating new video: {}", req.title);
    
    match req.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(&serde_json::json!({
            "error": format!("Validation error: {}", e)
        })),
    }

    match db::create_video(data.get_ref(), req.title.clone(), req.youtube_id.clone()).await {
        Ok(video) => HttpResponse::Created().json(&VideoResponse {
            id: video.id,
            title: video.title,
            youtube_id: video.youtube_id,
            created_at: video.created_at,
            updated_at: video.updated_at,
            deleted_at: video.deleted_at,
        }),
        Err(e) => HttpResponse::InternalServerError().json(&serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

#[web::get("/videos")]
pub async fn list_videos(
    query: Query<VideoQuery>,
    data: State<DatabaseConnection>,
) -> impl Responder {
    info!("Listing videos with query: {:?}", query);
    
    match db::list_videos(data.get_ref(), query.into_inner()).await {
        Ok(result) => {
            let videos = result.videos.into_iter().map(|v| VideoResponse {
                id: v.id,
                title: v.title,
                youtube_id: v.youtube_id,
                created_at: v.created_at,
                updated_at: v.updated_at,
                deleted_at: v.deleted_at,
            }).collect();

            HttpResponse::Ok().json(&PaginatedVideoResponse {
                videos,
                total: result.total,
                page: result.page,
                per_page: result.per_page,
                total_pages: result.total_pages,
            })
        },
        Err(e) => HttpResponse::InternalServerError().json(&serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

#[web::get("/videos/{id}")]
pub async fn get_video(
    id: Path<i32>,
    data: State<DatabaseConnection>,
) -> impl Responder {
    info!("Getting video with id: {}", id);
    
    match db::get_video(data.get_ref(), *id).await {
        Ok(Some(video)) => HttpResponse::Ok().json(&VideoResponse {
            id: video.id,
            title: video.title,
            youtube_id: video.youtube_id,
            created_at: video.created_at,
            updated_at: video.updated_at,
            deleted_at: video.deleted_at,
        }),
        Ok(None) => HttpResponse::NotFound().json(&serde_json::json!({
            "error": format!("Video with id {} not found", id)
        })),
        Err(e) => HttpResponse::InternalServerError().json(&serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

#[web::put("/videos/{id}")]
pub async fn update_video(
    id: Path<i32>,
    req: Json<UpdateVideoRequest>,
    data: State<DatabaseConnection>,
) -> impl Responder {
    info!("Updating video with id: {}", id);
    
    match req.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(&serde_json::json!({
            "error": format!("Validation error: {}", e)
        })),
    }

    match db::update_video(data.get_ref(), *id, req.title.clone(), req.youtube_id.clone()).await {
        Ok(Some(video)) => HttpResponse::Ok().json(&VideoResponse {
            id: video.id,
            title: video.title,
            youtube_id: video.youtube_id,
            created_at: video.created_at,
            updated_at: video.updated_at,
            deleted_at: video.deleted_at,
        }),
        Ok(None) => HttpResponse::NotFound().json(&serde_json::json!({
            "error": format!("Video with id {} not found", id)
        })),
        Err(e) => HttpResponse::InternalServerError().json(&serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

#[web::delete("/videos/{id}")]
pub async fn delete_video(
    id: Path<i32>,
    data: State<DatabaseConnection>,
) -> impl Responder {
    info!("Deleting video with id: {}", id);
    
    match db::delete_video(data.get_ref(), *id).await {
        Ok(true) => HttpResponse::Ok().json(&serde_json::json!({
            "message": format!("Video with id {} deleted", id)
        })),
        Ok(false) => HttpResponse::NotFound().json(&serde_json::json!({
            "error": format!("Video with id {} not found", id)
        })),
        Err(e) => HttpResponse::InternalServerError().json(&serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntex::web::test;
    use chrono::Utc;

    #[ntex::test]
    async fn test_create_video() {
        let db = db::init_db().await;
        let app = test::init_service(
            web::App::new()
                .state(db)
                .configure(crate::app::config_app),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/videos")
            .set_json(&CreateVideoRequest {
                title: "Test Video".to_string(),
                youtube_id: "dQw4w9WgXcQ".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);

        let body = test::read_body(resp).await;
        let video: VideoResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(video.title, "Test Video");
        assert_eq!(video.youtube_id, "dQw4w9WgXcQ");
        assert!(video.id > 0);
        assert!(video.created_at <= Utc::now());
        assert!(video.updated_at <= Utc::now());
        assert!(video.deleted_at.is_none());
    }
}
