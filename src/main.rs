mod commands;
mod database;
mod import;
mod settings;

use anyhow::Result;
use entity::prelude::*;
use import::getpocket as GetPocketExtractor;
use sea_orm::{entity::Set, EntityTrait};

#[async_std::main]
async fn main() -> Result<()> {
    import().await;
    list_all().await;
    Ok(())
}

pub async fn import() {
    let connection = database::get_connection().await;
    let getpocket_links = GetPocketExtractor::get_links();

    for link in getpocket_links {
        let url = UrlActiveModel {
            url: Set(link.url),
            ..Default::default()
        };
        Url::insert(url).exec(&connection).await.unwrap();
    }
}
pub async fn list_all() {
    let connection = database::get_connection().await;
    let urls = Url::find().all(&connection).await;
    println!("{:#?}", urls);
}
