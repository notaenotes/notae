pub use sea_orm_migration::prelude::*;

mod m20220513_181235_create_urls_table;
mod m20220513_182027_create_tags_table;
mod m20220521_013302_create_urls_tags_table;
mod m20220528_194635_create_urls_content_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220513_181235_create_urls_table::Migration),
            Box::new(m20220513_182027_create_tags_table::Migration),
            Box::new(m20220521_013302_create_urls_tags_table::Migration),
            Box::new(m20220528_194635_create_urls_content_table::Migration),
        ]
    }
}
