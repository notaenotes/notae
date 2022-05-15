use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

use migration::Migrator;

pub async fn get_connection() -> DatabaseConnection {
    dotenv().ok();

    migration::cli::run_cli(Migrator).await;

    let database_url = env::var("DATABASE_URL").unwrap();
    let connection = Database::connect(database_url).await;
    connection.unwrap()
}
