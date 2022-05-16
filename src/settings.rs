use config::Config;
use dirs;
use std::{collections::HashMap, path::Path};

pub fn get_settings_directory() -> std::path::PathBuf {
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

pub fn get_settings_file() -> String {
    get_settings_directory().to_str().unwrap().to_string() + "/config.toml"
}

pub fn get_settings() -> HashMap<String, String> {
    let settings_file_path = get_settings_file();
    let settings = Config::builder()
        .add_source(config::File::with_name(&settings_file_path))
        .build();

    settings
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
}
