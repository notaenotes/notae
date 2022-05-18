use config::Config;
use dirs;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub enum DirectoryType {
    Config,
    Data,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Settings {
    DATABASE_URL: String,
}

pub fn get_settings_directory(directory_type: DirectoryType) -> std::path::PathBuf {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let mut settings_dir_path = match directory_type {
        DirectoryType::Config => dirs::config_dir(),
        DirectoryType::Data => dirs::data_dir(),
    }
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap();

    settings_dir_path += "/";
    settings_dir_path += &crate_name;

    Path::new(&settings_dir_path).to_owned()
}

pub fn create_settings_directory(directory_name: PathBuf) {
    fs::create_dir(directory_name).unwrap();
}

fn generate_connection_string() -> String {
    let database_directory = get_settings_directory(DirectoryType::Data)
        .to_str()
        .unwrap()
        .to_owned();

    let mut connection_string = String::from("sqlite:://");
    connection_string.push_str(&database_directory);
    connection_string.push_str("/notae.db?mode=rwc");

    connection_string
}

pub fn create_settings_file(file_path: PathBuf, force: &bool) {
    let settings = Settings {
        DATABASE_URL: generate_connection_string(),
    };

    let toml_content = toml::to_string(&settings).unwrap();
    fs::write(file_path, toml_content).unwrap();
}

pub fn get_settings_file() -> PathBuf {
    let file_path = get_settings_directory(DirectoryType::Config)
        .to_str()
        .unwrap()
        .to_string()
        + "/config.toml";

    return Path::new(&file_path).to_owned();
}

pub fn get_settings() -> HashMap<String, String> {
    let settings_file_path = get_settings_file().into_os_string().into_string().unwrap();

    let settings = Config::builder()
        .add_source(config::File::with_name(&settings_file_path))
        .build();

    settings
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
}
