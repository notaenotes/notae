use crate::database;
use entity::prelude::*;
use sea_orm::EntityTrait;

pub async fn list_all() {
    let connection = database::get_connection().await;
    let urls = Url::find().all(&connection).await;
    println!("{:#?}", urls);
}
