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
#[derive(Clone)]
pub struct VideoService {
    repository: VideoRepository,
}

impl VideoService {
    /// Creates a new instance of VideoService
    /// 
    /// # Arguments
    /// * `repository` - The video repository instance for data access
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
    /// * `AppResult<PaginatedVideoResponse>` - List of videos matching the query
    /// 
    /// # Errors
    /// * `AppError::Database` - If there's an error accessing the database
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
