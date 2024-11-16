use sea_orm::*;
use crate::entity::video;
use crate::error::{AppError, AppResult};
use crate::db::VideoQuery;

#[derive(Clone)]
pub struct VideoRepository {
    db: DatabaseConnection,
}

impl VideoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, title: String, youtube_id: String) -> AppResult<video::Model> {
        let video = video::ActiveModel {
            title: Set(title),
            youtube_id: Set(youtube_id),
            ..Default::default()
        };

        let result = video.insert(&self.db).await?;
        Ok(result)
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<Option<video::Model>> {
        let video = video::Entity::find_by_id(id)
            .filter(video::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?;
        Ok(video)
    }

    pub async fn update(&self, id: i32, title: Option<String>, youtube_id: Option<String>) -> AppResult<Option<video::Model>> {
        let video = self.find_by_id(id).await?;
        
        if let Some(mut video) = video {
            let mut video: video::ActiveModel = video.into();
            
            if let Some(title) = title {
                video.title = Set(title);
            }
            if let Some(youtube_id) = youtube_id {
                video.youtube_id = Set(youtube_id);
            }
            
            let updated_video = video.update(&self.db).await?;
            Ok(Some(updated_video))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, id: i32) -> AppResult<bool> {
        let video = self.find_by_id(id).await?;
        
        if let Some(video) = video {
            let mut video: video::ActiveModel = video.into();
            video.deleted_at = Set(Some(chrono::Utc::now()));
            video.update(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn list(&self, query: &VideoQuery) -> AppResult<(Vec<video::Model>, u64)> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        
        let mut db_query = video::Entity::find()
            .filter(video::Column::DeletedAt.is_null());
            
        if let Some(ref search) = query.search {
            db_query = db_query.filter(video::Column::Title.contains(search));
        }

        let order_by = query.order_by.as_deref().unwrap_or("created_at");
        let order_direction = query.order_direction.as_deref().unwrap_or("desc");

        db_query = match (order_by, order_direction) {
            ("created_at", "desc") => db_query.order_by_desc(video::Column::CreatedAt),
            ("created_at", _) => db_query.order_by_asc(video::Column::CreatedAt),
            ("title", "desc") => db_query.order_by_desc(video::Column::Title),
            ("title", _) => db_query.order_by_asc(video::Column::Title),
            _ => db_query.order_by_desc(video::Column::CreatedAt),
        };

        let paginator = db_query.paginate(&self.db, per_page);
        let total = paginator.num_items().await?;
        let videos = paginator.fetch_page(page - 1).await?;

        Ok((videos, total))
    }
}
