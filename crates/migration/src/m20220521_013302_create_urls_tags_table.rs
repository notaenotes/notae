use sea_orm_migration::prelude::*;

pub struct Migration;

use entity::tag;
use entity::url;
use entity::url_tag;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220521_013302_create_urls_tags_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(url_tag::Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(url_tag::Column::IdUrl).integer().not_null())
                    .col(ColumnDef::new(url_tag::Column::IdTag).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-urls-tags")
                            .col(url_tag::Column::IdUrl)
                            .col(url_tag::Column::IdTag),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-urls-tags-id-url")
                            .from(url_tag::Entity, url_tag::Column::IdUrl)
                            .to(url::Entity, url::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-urls-tags-id-tag")
                            .from(url_tag::Entity, url_tag::Column::IdTag)
                            .to(tag::Entity, tag::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(url_tag::Entity).to_owned())
            .await
    }
}
