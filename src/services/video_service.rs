use crate::api::{CreateVideoRequest, UpdateVideoRequest, VideoResponse, PaginatedVideoResponse};
use crate::db::VideoQuery;
use crate::error::{AppError, AppResult};
use crate::repositories::video_repository::VideoRepository;
use validator::Validate;

/// Service layer for handling video business logic
/// 
/// This service implements the business logic for video operations, including:
/// - Input validation
/// - Data transformation
/// - Business rules enforcement
/// - Coordination with the repository layer
/// 
/// # Examples
/// 
/// ```no_run
/// use ntex_api::services::video_service::VideoService;
/// use ntex_api::repositories::video_repository::VideoRepository;
/// use sea_orm::DatabaseConnection;
/// 
/// async fn setup(db: DatabaseConnection) {
///     // Create a new video service
///     let repo = VideoRepository::new(db);
///     let service = VideoService::new(repo);
/// }
/// ```
#[derive(Clone)]
pub struct VideoService {
    repository: VideoRepository,
}

impl VideoService {
    /// Creates a new instance of VideoService
    /// 
    /// # Arguments
    /// * `repository` - The video repository instance for data access
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn setup(db: DatabaseConnection) {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    /// }
    /// ```
    pub fn new(repository: VideoRepository) -> Self {
        Self { repository }
    }

    /// Creates a new video
    /// 
    /// # Arguments
    /// * `req` - The video creation request containing title and youtube_id
    /// 
    /// # Returns
    /// * `AppResult<VideoResponse>` - The created video on success
    /// 
    /// # Errors
    /// * `AppError::Validation` - If the input data is invalid
    /// * `AppError::Database` - If there's an error saving to the database
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use ntex_api::api::CreateVideoRequest;
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn create_video(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    /// 
    ///     let request = CreateVideoRequest {
    ///         title: "My Awesome Video".to_string(),
    ///         youtube_id: "dQw4w9WgXcQ".to_string(),
    ///     };
    /// 
    ///     let video = service.create_video(request).await?;
    ///     assert_eq!(video.title, "My Awesome Video");
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_video(&self, req: CreateVideoRequest) -> AppResult<VideoResponse> {
        if let Err(e) = req.validate() {
            return Err(AppError::Validation(e.to_string()));
        }

        let video = self.repository.create(req.title, req.youtube_id).await?;
        Ok(VideoResponse {
            id: video.id,
            title: video.title,
            youtube_id: video.youtube_id,
            created_at: video.created_at,
            updated_at: video.updated_at,
            deleted_at: video.deleted_at,
        })
    }

    /// Retrieves a video by ID
    /// 
    /// # Arguments
    /// * `id` - The ID of the video to retrieve
    /// 
    /// # Returns
    /// * `AppResult<VideoResponse>` - The requested video on success
    /// 
    /// # Errors
    /// * `AppError::NotFound` - If the video doesn't exist
    /// * `AppError::Database` - If there's an error accessing the database
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn get_video(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    ///     
    ///     let video = service.get_video(1).await?;
    ///     assert_eq!(video.id, 1);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_video(&self, id: i32) -> AppResult<VideoResponse> {
        let video = self.repository.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Video with id {} not found", id)))?;
            
        Ok(VideoResponse {
            id: video.id,
            title: video.title,
            youtube_id: video.youtube_id,
            created_at: video.created_at,
            updated_at: video.updated_at,
            deleted_at: video.deleted_at,
        })
    }

    /// Updates an existing video
    /// 
    /// # Arguments
    /// * `id` - The ID of the video to update
    /// * `req` - The video update request containing new title and youtube_id
    /// 
    /// # Returns
    /// * `AppResult<VideoResponse>` - The updated video on success
    /// 
    /// # Errors
    /// * `AppError::NotFound` - If the video doesn't exist
    /// * `AppError::Validation` - If the input data is invalid
    /// * `AppError::Database` - If there's an error updating the database
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use ntex_api::api::UpdateVideoRequest;
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn update_video(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    /// 
    ///     let request = UpdateVideoRequest {
    ///         title: Some("Updated Video Title".to_string()),
    ///         youtube_id: Some("dQw4w9WgXcQ".to_string()),
    ///     };
    /// 
    ///     let video = service.update_video(1, request).await?;
    ///     assert_eq!(video.title, "Updated Video Title");
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_video(&self, id: i32, req: UpdateVideoRequest) -> AppResult<VideoResponse> {
        if let Err(e) = req.validate() {
            return Err(AppError::Validation(e.to_string()));
        }

        let video = self.repository.update(id, req.title, req.youtube_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Video with id {} not found", id)))?;
            
        Ok(VideoResponse {
            id: video.id,
            title: video.title,
            youtube_id: video.youtube_id,
            created_at: video.created_at,
            updated_at: video.updated_at,
            deleted_at: video.deleted_at,
        })
    }

    /// Deletes a video
    /// 
    /// # Arguments
    /// * `id` - The ID of the video to delete
    /// 
    /// # Returns
    /// * `AppResult<bool>` - Success indicator
    /// 
    /// # Errors
    /// * `AppError::NotFound` - If the video doesn't exist
    /// * `AppError::Database` - If there's an error deleting from the database
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn delete_video(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    /// 
    ///     service.delete_video(1).await?;
    /// 
    ///     // Verify deletion
    ///     let result = service.get_video(1).await;
    ///     assert!(result.is_err());
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_video(&self, id: i32) -> AppResult<bool> {
        let deleted = self.repository.delete(id).await?;
        if !deleted {
            return Err(AppError::NotFound(format!("Video with id {} not found", id)));
        }
        Ok(true)
    }

    /// Lists videos based on query parameters
    /// 
    /// # Arguments
    /// * `query` - Query parameters for filtering and pagination
    /// 
    /// # Returns
    /// * `AppResult<PaginatedVideoResponse>` - The paginated list of videos on success
    /// 
    /// # Errors
    /// * `AppError::Database` - If there's an error accessing the database
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use ntex_api::db::VideoQuery;
    /// use ntex_api::services::video_service::VideoService;
    /// use ntex_api::repositories::video_repository::VideoRepository;
    /// use sea_orm::DatabaseConnection;
    /// 
    /// async fn list_videos(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    ///     let repo = VideoRepository::new(db);
    ///     let service = VideoService::new(repo);
    /// 
    ///     let query = VideoQuery {
    ///         page: Some(1),
    ///         per_page: Some(10),
    ///         search: Some("awesome".to_string()),
    ///         order_by: Some("created_at".to_string()),
    ///         order_direction: Some("desc".to_string()),
    ///     };
    /// 
    ///     let videos = service.list_videos(query).await?;
    ///     assert_eq!(videos.page, 1);
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_videos(&self, query: VideoQuery) -> AppResult<PaginatedVideoResponse> {
        let (videos, total) = self.repository.list(&query).await?;
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        let total_pages = (total as f64 / per_page as f64).ceil() as u64;

        let videos = videos.into_iter()
            .map(|v| VideoResponse {
                id: v.id,
                title: v.title,
                youtube_id: v.youtube_id,
                created_at: v.created_at,
                updated_at: v.updated_at,
                deleted_at: v.deleted_at,
            })
            .collect();

        Ok(PaginatedVideoResponse {
            videos,
            total,
            page,
            per_page,
            total_pages,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Database, Schema, DatabaseConnection, DatabaseBackend, ConnectionTrait};
    use crate::entity::video;

    async fn setup_database(db: &DatabaseConnection) {
        let schema = Schema::new(DatabaseBackend::Sqlite);
        let stmt = schema.create_table_from_entity(video::Entity);
        db.execute(db.get_database_backend().build(&stmt)).await.unwrap();
    }

    #[ntex::test]
    async fn test_create_video() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        setup_database(&db).await;
        let repo = VideoRepository::new(db);
        let service = VideoService::new(repo);

        let request = CreateVideoRequest {
            title: "Test Video".to_string(),
            youtube_id: "dQw4w9WgXcQ".to_string(), // 11 characters
        };

        let result = service.create_video(request).await;
        assert!(result.is_ok());
    }

    #[ntex::test]
    async fn test_get_video() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        setup_database(&db).await;
        let repo = VideoRepository::new(db);
        let service = VideoService::new(repo);

        let request = CreateVideoRequest {
            title: "Test Video".to_string(),
            youtube_id: "dQw4w9WgXcQ".to_string(), // 11 characters
        };

        let video = service.create_video(request).await.unwrap();
        let result = service.get_video(video.id).await;
        assert!(result.is_ok());
    }

    #[ntex::test]
    async fn test_list_videos() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        setup_database(&db).await;
        let repo = VideoRepository::new(db);
        let service = VideoService::new(repo);

        // 创建一些测试视频
        for i in 0..3 {
            let request = CreateVideoRequest {
                title: format!("Test Video {}", i),
                youtube_id: format!("dQw4w9WgXc{}", i), // 11 characters
            };
            service.create_video(request).await.unwrap();
        }

        let result = service.list_videos(VideoQuery::default()).await;
        assert!(result.is_ok());
        let videos = result.unwrap();
        assert_eq!(videos.videos.len(), 3);
    }

    #[ntex::test]
    async fn test_update_video() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        setup_database(&db).await;
        let repo = VideoRepository::new(db);
        let service = VideoService::new(repo);

        let request = CreateVideoRequest {
            title: "Test Video".to_string(),
            youtube_id: "dQw4w9WgXcQ".to_string(), // 11 characters
        };

        let video = service.create_video(request).await.unwrap();
        let update_request = UpdateVideoRequest {
            title: Some("Updated Video".to_string()),
            youtube_id: Some("xQc9WgXw4Qd".to_string()), // 11 characters
        };

        let result = service.update_video(video.id, update_request).await;
        assert!(result.is_ok());

        let updated = service.get_video(video.id).await.unwrap();
        assert_eq!(updated.title, "Updated Video");
        assert_eq!(updated.youtube_id, "xQc9WgXw4Qd");
    }

    #[ntex::test]
    async fn test_delete_video() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        setup_database(&db).await;
        let repo = VideoRepository::new(db);
        let service = VideoService::new(repo);

        let request = CreateVideoRequest {
            title: "Test Video".to_string(),
            youtube_id: "dQw4w9WgXcQ".to_string(), // 11 characters
        };

        let video = service.create_video(request).await.unwrap();
        let result = service.delete_video(video.id).await;
        assert!(result.is_ok());

        let get_result = service.get_video(video.id).await;
        assert!(get_result.is_err());
    }
}
