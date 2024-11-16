use sea_orm::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "videos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub youtube_id: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn init_db() -> DatabaseConnection {
    let db_url = "sqlite:./videos.db?mode=rwc";
    let db = Database::connect(db_url).await.expect("Failed to connect to database");
    
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);
    let mut table = schema.create_table_from_entity(Entity);
    let stmt = table.if_not_exists();
    db.execute(backend.build(stmt))
        .await
        .expect("Failed to create table");
    
    db
}

pub async fn create_video(db: &DatabaseConnection, title: String, youtube_id: String) -> Result<Model, DbErr> {
    let video = ActiveModel {
        title: Set(title),
        youtube_id: Set(youtube_id),
        created_at: Set(Utc::now().to_rfc3339()),
        ..Default::default()
    };

    video.insert(db).await
}

pub async fn list_videos(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(db).await
}
