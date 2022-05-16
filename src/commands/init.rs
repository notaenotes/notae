use std::{
    fs,
    path::{Path, PathBuf},
};

use dirs;

use crate::database;

pub async fn init() {
    let settings_directory_path: PathBuf = get_settings_directory();

    if !settings_directory_path.exists() {
        create_settings_directory(settings_directory_path);
    }

    println!("{}", get_settings_file());
    // TODO Create database first
    // database::get_connection().await;
}

fn get_settings_directory() -> std::path::PathBuf {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let mut settings_dir_path = dirs::config_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    settings_dir_path += "/";
    settings_dir_path += &crate_name;

    Path::new(&settings_dir_path).to_owned()
}

fn create_settings_directory(directory_name: PathBuf) {
    fs::create_dir(directory_name).unwrap();
}

fn get_settings_file() -> String {
    get_settings_directory().to_str().unwrap().to_string() + "/config.toml"
}
