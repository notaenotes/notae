pub use sea_orm_migration::prelude::*;

mod m20220513_181235_create_url_table;
mod m20220513_182027_create_tag_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220513_181235_create_url_table::Migration),
            Box::new(m20220513_182027_create_tag_table::Migration),
        ]
    }
}
