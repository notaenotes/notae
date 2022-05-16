use crate::database;
use crate::settings::{get_settings_directory, get_settings_file};
use migration::cli;
use std::{fs, path::PathBuf};

pub async fn init() {
    let settings_directory_path: PathBuf = get_settings_directory();

    if !settings_directory_path.exists() {
        create_settings_directory(settings_directory_path);
    }

    database::get_connection().await;

    cli::run_cli(migration::Migrator).await;
}

fn create_settings_directory(directory_name: PathBuf) {
    fs::create_dir(directory_name).unwrap();
}
