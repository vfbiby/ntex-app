use sea_orm::{
    Database, DatabaseConnection,
    DbErr, EntityTrait, Set, ActiveModelTrait,
    Condition, QueryFilter, PaginatorTrait, QuerySelect, ColumnTrait,
    QueryOrder,
};
use serde::Deserialize;
use chrono::Utc;
use migration::MigratorTrait;

use crate::entity::video::{self, Entity as Video, Model, ActiveModel};

#[derive(Debug, Deserialize)]
pub struct VideoQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub search: Option<String>,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
}

impl Default for VideoQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
            search: None,
            order_by: Some("created_at".to_string()),
            order_direction: Some("desc".to_string()),
        }
    }
}

pub struct PaginatedVideos {
    pub videos: Vec<Model>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

pub async fn init_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    migration::Migrator::up(&db, None).await.unwrap();
    db
}

pub async fn create_video(
    db: &DatabaseConnection,
    title: String,
    youtube_id: String,
) -> Result<Model, DbErr> {
    let video = ActiveModel {
        title: Set(title),
        youtube_id: Set(youtube_id),
        ..Default::default()
    };

    video.insert(db).await
}

pub async fn list_videos(
    db: &DatabaseConnection,
    query: VideoQuery,
) -> Result<PaginatedVideos, DbErr> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let search = query.search.unwrap_or_default();
    let order_by = query.order_by.unwrap_or_else(|| "created_at".to_string());
    let order_direction = query.order_direction.unwrap_or_else(|| "desc".to_string());

    let mut condition = Condition::all();
    
    // Only include non-deleted videos
    condition = condition.add(video::Column::DeletedAt.is_null());

    // Add search condition if search is not empty
    if !search.is_empty() {
        condition = condition
            .add(video::Column::Title.contains(&search))
            .add(video::Column::YoutubeId.contains(&search));
    }

    let mut select = Video::find().filter(condition);

    // Add ordering
    let order_by_col = match order_by.as_str() {
        "title" => video::Column::Title,
        "youtube_id" => video::Column::YoutubeId,
        _ => video::Column::CreatedAt,
    };

    select = match order_direction.as_str() {
        "asc" => select.order_by_asc(order_by_col),
        _ => select.order_by_desc(order_by_col),
    };

    // Get total count
    let total = select.clone().count(db).await?;

    // Calculate pagination
    let total_pages = (total + per_page - 1) / per_page;
    let offset = (page - 1) * per_page;

    // Get paginated videos
    let videos = select
        .offset(offset)
        .limit(per_page)
        .all(db)
        .await?;

    Ok(PaginatedVideos {
        videos,
        total,
        page,
        per_page,
        total_pages,
    })
}

pub async fn get_video(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<Model>, DbErr> {
    Video::find_by_id(id)
        .filter(video::Column::DeletedAt.is_null())
        .one(db)
        .await
}

pub async fn update_video(
    db: &DatabaseConnection,
    id: i32,
    title: Option<String>,
    youtube_id: Option<String>,
) -> Result<Option<Model>, DbErr> {
    let video = Video::find_by_id(id)
        .filter(video::Column::DeletedAt.is_null())
        .one(db)
        .await?;

    if let Some(video) = video {
        let mut active_model: ActiveModel = video.clone().into();

        if let Some(title) = title {
            active_model.title = Set(title);
        }

        if let Some(youtube_id) = youtube_id {
            active_model.youtube_id = Set(youtube_id);
        }

        let updated_video = active_model.update(db).await?;
        Ok(Some(updated_video))
    } else {
        Ok(None)
    }
}

pub async fn delete_video(
    db: &DatabaseConnection,
    id: i32,
) -> Result<bool, DbErr> {
    let video = Video::find_by_id(id)
        .filter(video::Column::DeletedAt.is_null())
        .one(db)
        .await?;

    if let Some(video) = video {
        let mut active_model: ActiveModel = video.into();
        active_model.deleted_at = Set(Some(Utc::now()));
        active_model.update(db).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
