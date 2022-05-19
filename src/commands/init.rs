use crate::settings::{get_settings_directory, get_settings_file, DirectoryType};
use crate::{database, settings};
use migration::{Migrator, MigratorTrait};
use std::path::PathBuf;

pub async fn init(force: &bool) {
    let settings_directory_path: PathBuf = get_settings_directory(DirectoryType::Config);
    let settings_data_path: PathBuf = get_settings_directory(DirectoryType::Data);
    let settings_file_path: PathBuf = get_settings_file();

    if !settings_directory_path.exists() {
        settings::create_settings_directory(settings_directory_path);
    }

    if !settings_data_path.exists() {
        settings::create_settings_directory(settings_data_path);
    }
    if !settings_file_path.exists() {
        settings::create_settings_file(settings_file_path, force);
    }

    // Get connection and run migrations
    let connection = database::get_connection().await;
    Migrator::up(&connection, None).await.unwrap();
}
