use sea_orm_migration::prelude::*;

use entity::{url, url_content};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220528_194635_create_urls_content_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(url_content::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(url_content::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(url_content::Column::IdUrl)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(url_content::Column::Hash)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(url_content::Column::Content)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(url_content::Column::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-urls-content-id-url")
                            .from(url_content::Entity, url_content::Column::IdUrl)
                            .to(url::Entity, url::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(url_content::Entity).to_owned())
            .await
    }
}
