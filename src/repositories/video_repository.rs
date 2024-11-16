use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, 
    PaginatorTrait, QueryFilter, QueryOrder, Set
};
use crate::entity::{video, video::Entity as Video};
use crate::error::{AppError, AppResult};
use crate::db::VideoQuery;
use chrono::{DateTime, Utc};

/// Repository layer for video data access
/// 
/// This repository handles all database operations for videos, including:
/// - CRUD operations
/// - Pagination
/// - Filtering
/// - Sorting
#[derive(Clone)]
pub struct VideoRepository {
    db: DatabaseConnection,
}

impl VideoRepository {
    /// Creates a new instance of VideoRepository
    /// 
    /// # Arguments
    /// * `db` - Database connection instance
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Creates a new video in the database
    /// 
    /// # Arguments
    /// * `title` - The title of the video
    /// * `youtube_id` - The YouTube ID of the video
    /// 
    /// # Returns
    /// * `AppResult<video::Model>` - The created video model
    /// 
    /// # Errors
    /// * `AppError::Database` - If there's an error executing the query
    pub async fn create(&self, title: String, youtube_id: String) -> AppResult<video::Model> {
        let video = video::ActiveModel {
            title: Set(title),
            youtube_id: Set(youtube_id),
            ..Default::default()
        };

        let video = Video::insert(video)
            .exec_with_returning(&self.db)
            .await
            .map_err(AppError::Database)?;

        Ok(video)
    }

    /// Finds a video by its ID
    /// 
    /// # Arguments
    /// * `id` - The ID of the video to find
    /// 
    /// # Returns
    /// * `AppResult<Option<video::Model>>` - The found video model, if any
    /// 
    /// # Errors
    /// * `AppError::Database` - If there's an error executing the query
    pub async fn find_by_id(&self, id: i32) -> AppResult<Option<video::Model>> {
        let video = Video::find_by_id(id)
            .filter(video::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
            .map_err(AppError::Database)?;

        Ok(video)
    }

    /// Updates an existing video
    /// 
    /// # Arguments
    /// * `id` - The ID of the video to update
    /// * `title` - The new title of the video
    /// * `youtube_id` - The new YouTube ID of the video
    /// 
    /// # Returns
    /// * `AppResult<Option<video::Model>>` - The updated video model, if found
    /// 
    /// # Errors
    /// * `AppError::Database` - If there's an error executing the query
    pub async fn update(&self, id: i32, title: Option<String>, youtube_id: Option<String>) -> AppResult<Option<video::Model>> {
        let video = self.find_by_id(id).await?;
        
        if let Some(video) = video {
            let mut video: video::ActiveModel = video.into();
            
            if let Some(title) = title {
                video.title = Set(title);
            }
            
            if let Some(youtube_id) = youtube_id {
                video.youtube_id = Set(youtube_id);
            }

            let updated_video = video.update(&self.db).await
                .map_err(AppError::Database)?;

            Ok(Some(updated_video))
        } else {
            Ok(None)
        }
    }

    /// Deletes a video by its ID
    /// 
    /// # Arguments
    /// * `id` - The ID of the video to delete
    /// 
    /// # Returns
    /// * `AppResult<bool>` - True if the video was deleted, false if not found
    /// 
    /// # Errors
    /// * `AppError::Database` - If there's an error executing the query
    pub async fn delete(&self, id: i32) -> AppResult<bool> {
        let video = self.find_by_id(id).await?;
        
        if let Some(video) = video {
            let mut video: video::ActiveModel = video.into();
            video.deleted_at = Set(Some(Utc::now()));
            video.update(&self.db).await.map_err(AppError::Database)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Lists videos with pagination and filtering
    /// 
    /// # Arguments
    /// * `query` - Query parameters for filtering and pagination
    /// 
    /// # Returns
    /// * `AppResult<(Vec<video::Model>, u64)>` - Tuple of videos and total count
    /// 
    /// # Errors
    /// * `AppError::Database` - If there's an error executing the query
    pub async fn list(&self, query: &VideoQuery) -> AppResult<(Vec<video::Model>, u64)> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        
        let mut db_query = Video::find()
            .filter(video::Column::DeletedAt.is_null());

        if let Some(search) = &query.search {
            db_query = db_query.filter(video::Column::Title.contains(search));
        }

        let paginator = db_query
            .order_by_desc(video::Column::CreatedAt)
            .paginate(&self.db, per_page);

        let total = paginator.num_items().await.map_err(AppError::Database)?;
        let videos = paginator
            .fetch_page(page - 1)
            .await
            .map_err(AppError::Database)?;

        Ok((videos, total))
    }
}
