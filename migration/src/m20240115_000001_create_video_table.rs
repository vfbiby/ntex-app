use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Video::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Video::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Video::Title).string().not_null())
                    .col(ColumnDef::new(Video::YoutubeId).string().not_null())
                    .col(ColumnDef::new(Video::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Video::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Video::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

        // Add indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_videos_youtube_id")
                    .table(Video::Table)
                    .col(Video::YoutubeId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_videos_title")
                    .table(Video::Table)
                    .col(Video::Title)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Video::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Video {
    Table,
    Id,
    Title,
    YoutubeId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
