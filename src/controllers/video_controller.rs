use ntex::web::{self, types::{Json, Path, Query}, HttpResponse, Responder};
use crate::services::video_service::VideoService;
use crate::api::{CreateVideoRequest, UpdateVideoRequest};
use crate::db::VideoQuery;
use crate::error::AppResult;
use std::sync::Arc;

/// Video controller that handles HTTP requests for video resources
/// 
/// This controller provides a RESTful API for managing video resources with the following endpoints:
/// - `POST /api/v1/videos`: Create a new video
/// - `GET /api/v1/videos`: List videos with pagination and search
/// - `GET /api/v1/videos/{id}`: Get a specific video by ID
/// - `PUT /api/v1/videos/{id}`: Update a specific video
/// - `DELETE /api/v1/videos/{id}`: Delete a specific video
#[derive(Clone)]
pub struct VideoController {
    service: VideoService,
}

impl VideoController {
    /// Creates a new instance of VideoController
    /// 
    /// # Arguments
    /// * `service` - The video service instance to handle business logic
    pub fn new(service: VideoService) -> Self {
        Self { service }
    }

    /// Creates a new video resource
    /// 
    /// # Arguments
    /// * `req` - JSON payload containing video creation data
    /// 
    /// # Returns
    /// * `AppResult<impl Responder>` - Returns the created video on success
    /// 
    /// # API Endpoint
    /// `POST /api/v1/videos`
    /// 
    /// # Request Body
    /// ```json
    /// {
    ///   "title": "string",
    ///   "description": "string",
    ///   "url": "string"
    /// }
    /// ```
    /// 
    /// # Response
    /// * `201 Created` - Video created successfully
    /// * `400 Bad Request` - Invalid request payload
    /// * `500 Internal Server Error` - Server error
    pub async fn create_video(&self, req: Json<CreateVideoRequest>) -> AppResult<impl Responder> {
        let video = self.service.create_video(req.into_inner()).await?;
        Ok(HttpResponse::Created().json(&video))
    }

    /// Retrieves a specific video by ID
    /// 
    /// # Arguments
    /// * `id` - Path parameter containing the video ID
    /// 
    /// # Returns
    /// * `AppResult<impl Responder>` - Returns the requested video on success
    /// 
    /// # API Endpoint
    /// `GET /api/v1/videos/{id}`
    /// 
    /// # Response
    /// * `200 OK` - Video found
    /// * `404 Not Found` - Video not found
    /// * `500 Internal Server Error` - Server error
    pub async fn get_video(&self, id: Path<i32>) -> AppResult<impl Responder> {
        let video = self.service.get_video(id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(&video))
    }

    /// Updates a specific video by ID
    /// 
    /// # Arguments
    /// * `id` - Path parameter containing the video ID
    /// * `req` - JSON payload containing video update data
    /// 
    /// # Returns
    /// * `AppResult<impl Responder>` - Returns the updated video on success
    /// 
    /// # API Endpoint
    /// `PUT /api/v1/videos/{id}`
    /// 
    /// # Request Body
    /// ```json
    /// {
    ///   "title": "string",
    ///   "description": "string",
    ///   "url": "string"
    /// }
    /// ```
    /// 
    /// # Response
    /// * `200 OK` - Video updated successfully
    /// * `400 Bad Request` - Invalid request payload
    /// * `404 Not Found` - Video not found
    /// * `500 Internal Server Error` - Server error
    pub async fn update_video(&self, id: Path<i32>, req: Json<UpdateVideoRequest>) -> AppResult<impl Responder> {
        let video = self.service.update_video(id.into_inner(), req.into_inner()).await?;
        Ok(HttpResponse::Ok().json(&video))
    }

    /// Deletes a specific video by ID
    /// 
    /// # Arguments
    /// * `id` - Path parameter containing the video ID
    /// 
    /// # Returns
    /// * `AppResult<impl Responder>` - Returns no content on success
    /// 
    /// # API Endpoint
    /// `DELETE /api/v1/videos/{id}`
    /// 
    /// # Response
    /// * `204 No Content` - Video deleted successfully
    /// * `404 Not Found` - Video not found
    /// * `500 Internal Server Error` - Server error
    pub async fn delete_video(&self, id: Path<i32>) -> AppResult<impl Responder> {
        self.service.delete_video(id.into_inner()).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    /// Lists videos with optional filtering and pagination
    /// 
    /// # Arguments
    /// * `query` - Query parameters for filtering and pagination
    /// 
    /// # Returns
    /// * `AppResult<impl Responder>` - Returns a list of videos on success
    /// 
    /// # API Endpoint
    /// `GET /api/v1/videos`
    /// 
    /// # Query Parameters
    /// * `page` - Page number (default: 1)
    /// * `per_page` - Items per page (default: 10)
    /// * `search` - Optional search term
    /// * `sort` - Sort field (default: "created_at")
    /// * `order` - Sort order ("asc" or "desc", default: "desc")
    /// 
    /// # Response
    /// * `200 OK` - List of videos
    /// * `400 Bad Request` - Invalid query parameters
    /// * `500 Internal Server Error` - Server error
    pub async fn list_videos(&self, query: Query<VideoQuery>) -> AppResult<impl Responder> {
        let videos = self.service.list_videos(query.into_inner()).await?;
        Ok(HttpResponse::Ok().json(&videos))
    }
}

/// Configures the video controller routes
/// 
/// # Arguments
/// * `cfg` - Service configuration
/// * `controller` - Video controller instance
pub fn config(cfg: &mut web::ServiceConfig, controller: VideoController) {
    let controller = Arc::new(controller);
    let c1 = controller.clone();
    let c2 = controller.clone();
    let c3 = controller.clone();
    let c4 = controller.clone();
    let c5 = controller.clone();
    
    cfg.service(
        web::scope("/api/v1/videos")
            .route("", web::post().to(move |req: Json<CreateVideoRequest>| {
                let ctrl = Arc::clone(&c1);
                async move { ctrl.create_video(req).await }
            }))
            .route("", web::get().to(move |query: Query<VideoQuery>| {
                let ctrl = Arc::clone(&c2);
                async move { ctrl.list_videos(query).await }
            }))
            .route("/{id}", web::get().to(move |id: Path<i32>| {
                let ctrl = Arc::clone(&c3);
                async move { ctrl.get_video(id).await }
            }))
            .route("/{id}", web::put().to(move |id: Path<i32>, req: Json<UpdateVideoRequest>| {
                let ctrl = Arc::clone(&c4);
                async move { ctrl.update_video(id, req).await }
            }))
            .route("/{id}", web::delete().to(move |id: Path<i32>| {
                let ctrl = Arc::clone(&c5);
                async move { ctrl.delete_video(id).await }
            }))
    );
}
