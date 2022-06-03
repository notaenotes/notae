use common::settings::{get_settings_directory, get_settings_file, DirectoryType};
use common::{database, settings};
use dialoguer::{theme::ColorfulTheme, Confirm};
use migration::{Migrator, MigratorTrait};
use std::path::PathBuf;

pub async fn init(force: &bool) {
    let settings_directory_path: PathBuf = get_settings_directory(DirectoryType::Config);
    let settings_data_path: PathBuf = get_settings_directory(DirectoryType::Data);
    let settings_file_path: PathBuf = get_settings_file();

    if !settings_directory_path.exists() {
        settings::create_settings_directory(&settings_directory_path);
    }

    if !settings_data_path.exists() {
        settings::create_settings_directory(&settings_data_path);
    }

    if *force && settings_directory_path.exists() {
        let overwrite_configurations_confirmation = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("This option will ERASE all your configurations. Do you want to continue?")
            .default(false)
            .show_default(false)
            .wait_for_newline(true)
            .interact()
            .unwrap();
        if overwrite_configurations_confirmation {
            settings::create_settings_file(&settings_file_path);
        }
    } else {
        settings::create_settings_file(&settings_file_path);
    }

    let connection = database::get_connection().await.unwrap_or_default();
    Migrator::up(&connection, None).await.unwrap();
}
