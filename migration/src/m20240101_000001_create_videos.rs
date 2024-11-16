use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Videos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Videos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Videos::Title).string().not_null())
                    .col(ColumnDef::new(Videos::YoutubeId).string().not_null())
                    .col(ColumnDef::new(Videos::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Videos::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Videos::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Videos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Videos {
    Table,
    Id,
    Title,
    YoutubeId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
