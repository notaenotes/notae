use anyhow::Result;
use sea_orm::{entity::Set, EntityTrait};
mod database;
mod import;

use import::getpocket as GetPocketExtractor;

use entity::prelude::*;

#[async_std::main]
async fn main() -> Result<()> {
    let connection = database::get_connection().await;
    let getpocket_links = GetPocketExtractor::get_links();

    for link in getpocket_links {
        let url = UrlActiveModel {
            url: Set(link.url),
            ..Default::default()
        };
        Url::insert(url).exec(&connection).await.unwrap();
    }

    Ok(())
}

pub async fn list_all() {
    let connection = database::get_connection().await;
    let urls = Url::find().all(&connection).await;
    println!("{:#?}", urls);
}
