use ntex::web::{self, types::{Json, Path, Query}, HttpResponse, Responder};
use crate::services::video_service::VideoService;
use crate::api::{CreateVideoRequest, UpdateVideoRequest};
use crate::db::VideoQuery;
use crate::error::AppResult;
use std::sync::Arc;

/// Video controller that handles HTTP requests for video resources
/// 
/// This controller provides a RESTful API for managing video resources.
/// 
/// # Example
/// 
/// ```no_run
/// use ntex::web;
/// use ntex_api::controllers::video_controller::VideoController;
/// use ntex_api::services::video_service::VideoService;
/// use ntex_api::repositories::video_repository::VideoRepository;
/// use sea_orm::DatabaseConnection;
/// 
/// async fn setup(db: DatabaseConnection) {
///     // Create a new video controller
///     let repo = VideoRepository::new(db);
///     let service = VideoService::new(repo);
///     let controller = VideoController::new(service);
/// 
///     // Configure routes
///     let app = web::App::new()
///         .configure(|cfg| {
///             let controller = std::sync::Arc::new(controller);
///             let c1 = controller.clone();
///             let c2 = controller.clone();
///             let c3 = controller.clone();
///             let c4 = controller.clone();
///             let c5 = controller.clone();
///             
///             cfg.service(
///                 web::scope("/api/v1/videos")
///                     .route("", web::post().to(move |req| {
///                         let ctrl = std::sync::Arc::clone(&c1);
///                         async move { ctrl.create_video(req).await }
///                     }))
///                     .route("", web::get().to(move |query| {
///                         let ctrl = std::sync::Arc::clone(&c2);
///                         async move { ctrl.list_videos(query).await }
///                     }))
///                     .route("/{id}", web::get().to(move |id| {
///                         let ctrl = std::sync::Arc::clone(&c3);
///                         async move { ctrl.get_video(id).await }
///                     }))
///                     .route("/{id}", web::put().to(move |id, req| {
///                         let ctrl = std::sync::Arc::clone(&c4);
///                         async move { ctrl.update_video(id, req).await }
///                     }))
///                     .route("/{id}", web::delete().to(move |id| {
///                         let ctrl = std::sync::Arc::clone(&c5);
///                         async move { ctrl.delete_video(id).await }
///                     }))
///             );
///         });
/// }
/// ```
#[derive(Clone)]
pub struct VideoController {
    service: VideoService,
}

impl VideoController {
    /// Creates a new instance of VideoController
    /// 
    /// # Arguments
    /// * `service` - The video service instance to handle business logic
    /// 
    /// # Example
    /// 
    /// ```no_run
/// use ntex_api::services::video_service::VideoService;
/// use ntex_api::controllers::video_controller::VideoController;
/// use ntex_api::repositories::video_repository::VideoRepository;
/// use sea_orm::DatabaseConnection;
/// 
/// async fn setup(db: DatabaseConnection) {
///     let repo = VideoRepository::new(db);
///     let service = VideoService::new(repo);
///     let controller = VideoController::new(service);
/// }
/// ```
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
    /// # Example
    /// 
    /// ```text
    /// POST /api/v1/videos
    /// Content-Type: application/json
    /// 
    /// {
    ///   "title": "My Awesome Video",
    ///   "youtube_id": "dQw4w9WgXcQ"
    /// }
    /// ```
    /// 
    /// ```text
    /// HTTP/1.1 201 Created
    /// Content-Type: application/json
    /// 
    /// {
    ///   "id": 1,
    ///   "title": "My Awesome Video",
    ///   "youtube_id": "dQw4w9WgXcQ",
    ///   "created_at": "2023-01-01T00:00:00Z",
    ///   "updated_at": "2023-01-01T00:00:00Z",
    ///   "deleted_at": null
    /// }
    /// ```
    /// 
    /// ```no_run
    /// use ntex::web::types::Json;
    /// use ntex_api::api::CreateVideoRequest;
    /// use ntex_api::controllers::video_controller::VideoController;
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn create_video(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    ///     let controller = VideoController::new(service);
    ///     
    ///     let request = CreateVideoRequest {
    ///         title: "My Awesome Video".to_string(),
    ///         youtube_id: "dQw4w9WgXcQ".to_string(),
    ///     };
    /// 
    ///     let response = controller.create_video(Json(request)).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_video(&self, req: Json<CreateVideoRequest>) -> AppResult<impl Responder> {
        let video = self.service.create_video(req.into_inner()).await?;
        Ok(HttpResponse::Created().json(&video))
    }

    /// Lists videos with optional filtering and pagination
    /// 
    /// # Arguments
    /// * `query` - Query parameters for filtering and pagination
    /// 
    /// # Returns
    /// * `AppResult<impl Responder>` - Returns a list of videos on success
    /// 
    /// # Example
    /// 
    /// ```text
    /// GET /api/v1/videos?page=1&per_page=10&search=awesome
    /// ```
    /// 
    /// ```text
    /// HTTP/1.1 200 OK
    /// Content-Type: application/json
    /// 
    /// {
    ///   "videos": [
    ///     {
    ///       "id": 1,
    ///       "title": "My Awesome Video",
    ///       "youtube_id": "dQw4w9WgXcQ",
    ///       "created_at": "2023-01-01T00:00:00Z",
    ///       "updated_at": "2023-01-01T00:00:00Z",
    ///       "deleted_at": null
    ///     }
    ///   ],
    ///   "total": 1,
    ///   "page": 1,
    ///   "per_page": 10,
    ///   "total_pages": 1
    /// }
    /// ```
    /// 
    /// ```no_run
    /// use ntex::web::types::{Query, Path};
    /// use ntex_api::db::VideoQuery;
    /// use ntex_api::controllers::video_controller::VideoController;
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn list_videos(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    ///     let controller = VideoController::new(service);
    ///     
    ///     let query = VideoQuery {
    ///         page: Some(1),
    ///         per_page: Some(10),
    ///         search: Some("awesome".to_string()),
    ///         order_by: Some("created_at".to_string()),
    ///         order_direction: Some("desc".to_string()),
    ///     };
    /// 
    ///     let response = controller.list_videos(Query(query)).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_videos(&self, query: Query<VideoQuery>) -> AppResult<impl Responder> {
        let videos = self.service.list_videos(query.into_inner()).await?;
        Ok(HttpResponse::Ok().json(&videos))
    }

    /// Retrieves a specific video by ID
    /// 
    /// # Arguments
    /// * `id` - Path parameter containing the video ID
    /// 
    /// # Returns
    /// * `AppResult<impl Responder>` - Returns the requested video on success
    /// 
    /// # Example
    /// 
    /// ```text
    /// GET /api/v1/videos/1
    /// ```
    /// 
    /// ```text
    /// HTTP/1.1 200 OK
    /// Content-Type: application/json
    /// 
    /// {
    ///   "id": 1,
    ///   "title": "My Awesome Video",
    ///   "youtube_id": "dQw4w9WgXcQ",
    ///   "created_at": "2023-01-01T00:00:00Z",
    ///   "updated_at": "2023-01-01T00:00:00Z",
    ///   "deleted_at": null
    /// }
    /// ```
    /// 
    /// ```no_run
    /// use ntex::web::types::Path;
    /// use ntex_api::controllers::video_controller::VideoController;
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn get_video(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    ///     let controller = VideoController::new(service);
    ///     
    ///     let id = 1i32;
    ///     let response = controller.get_video(id.into()).await?;
    ///     Ok(())
    /// }
    /// ```
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
    /// # Example
    /// 
    /// ```text
    /// PUT /api/v1/videos/1
    /// Content-Type: application/json
    /// 
    /// {
    ///   "title": "Updated Video Title",
    ///   "youtube_id": "dQw4w9WgXcQ"
    /// }
    /// ```
    /// 
    /// ```text
    /// HTTP/1.1 200 OK
    /// Content-Type: application/json
    /// 
    /// {
    ///   "id": 1,
    ///   "title": "Updated Video Title",
    ///   "youtube_id": "dQw4w9WgXcQ",
    ///   "created_at": "2023-01-01T00:00:00Z",
    ///   "updated_at": "2023-01-01T00:00:00Z",
    ///   "deleted_at": null
    /// }
    /// ```
    /// 
    /// ```no_run
    /// use ntex::web::types::{Path, Json};
    /// use ntex_api::api::UpdateVideoRequest;
    /// use ntex_api::controllers::video_controller::VideoController;
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn update_video(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    ///     let controller = VideoController::new(service);
    ///     
    ///     let request = UpdateVideoRequest {
    ///         title: Some("Updated Video Title".to_string()),
    ///         youtube_id: Some("dQw4w9WgXcQ".to_string()),
    ///     };
    /// 
    ///     let id = 1i32;
    ///     let response = controller.update_video(id.into(), Json(request)).await?;
    ///     Ok(())
    /// }
    /// ```
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
    /// # Example
    /// 
    /// ```text
    /// DELETE /api/v1/videos/1
    /// ```
    /// 
    /// ```text
    /// HTTP/1.1 204 No Content
    /// ```
    /// 
    /// ```no_run
    /// use ntex::web::types::Path;
    /// use ntex_api::controllers::video_controller::VideoController;
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn delete_video(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    ///     let controller = VideoController::new(service);
    ///     
    ///     let id = 1i32;
    ///     let response = controller.delete_video(id.into()).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_video(&self, id: Path<i32>) -> AppResult<impl Responder> {
        self.service.delete_video(id.into_inner()).await?;
        Ok(HttpResponse::NoContent().finish())
    }
}

/// Configures the video controller routes
/// 
/// # Arguments
/// * `cfg` - Service configuration
/// * `controller` - Video controller instance
/// 
/// # Example
/// 
/// ```no_run
/// use ntex::web;
/// use ntex_api::controllers::video_controller::{self, VideoController};
/// use ntex_api::services::video_service::VideoService;
/// use ntex_api::repositories::video_repository::VideoRepository;
/// use sea_orm::DatabaseConnection;
/// 
/// async fn setup(db: DatabaseConnection) {
///     let repo = VideoRepository::new(db);
///     let service = VideoService::new(repo);
///     let controller = VideoController::new(service);
///     
///     let app = web::App::new()
///         .configure(|cfg| {
///             video_controller::config(cfg, controller);
///         });
/// }
/// ```
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
