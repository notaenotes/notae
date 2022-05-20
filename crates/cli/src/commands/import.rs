use crate::import::getpocket::NewUrl;
use common::database;
use entity::prelude::*;
use sea_orm::{entity::Set, EntityTrait};
use std::path::PathBuf;

use crate::import;

pub async fn init(file_path: &Option<PathBuf>) {
    let captured_links = import::getpocket::get_links(&file_path.to_owned().unwrap());
    insert_captured_links(captured_links).await;
}
async fn insert_captured_links(captured_links: Vec<NewUrl>) {
    let connection = database::get_connection().await;

    for link in captured_links {
        let url = UrlActiveModel {
            url: Set(link.url),
            ..Default::default()
        };
        Url::insert(url).exec(&connection).await.unwrap();
    }
}
