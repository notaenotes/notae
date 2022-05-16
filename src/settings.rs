use std::collections::HashMap;

use config::Config;

const SETTINGS_FILE: &str = "config.toml";

pub fn get_settings() -> HashMap<String, String> {
    let settings = Config::builder()
        .add_source(config::File::with_name(SETTINGS_FILE))
        .build();

    settings
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
}
