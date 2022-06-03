use common::dal::get_entities;
use common::database::get_connection;
use entity::{url, url_content};
use rocket::{
    get,
    serde::json::{json, Value},
};
use sea_orm::{ActiveModelTrait, EntityTrait, JsonValue, Set};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct UrlTagResponse {
    #[serde(flatten)]
    url: entity::url::Model,
    tags: Vec<entity::tag::Model>,
}

#[get("/url")]
pub async fn get_urls() -> JsonValue {
    json!(get_entities::<url::Entity>().await)
}

#[get("/url/<id_url>")]
pub async fn get_url_by_id(id_url: i32) -> Value {
    let connection = get_connection().await.unwrap_or_default();
    let model = url::Entity::find_by_id_with_related_tags(id_url)
        .all(&connection)
        .await
        .unwrap_or_default()[0]
        .to_owned();

    json!(UrlTagResponse {
        url: model.0,
        tags: model.1,
    })
}

#[get("/url/<id_url>/get_content")]
pub async fn url_scrap(id_url: i32) {
    let connection = get_connection().await.unwrap_or_default();
    let model = url::Entity::find_by_id(id_url)
        .one(&connection)
        .await
        .unwrap_or_default()
        .unwrap();
    let body = reqwest::get(&model.url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_default();
    let compressed = smaz::compress(body.as_bytes());
    let url_content_active_model = url_content::ActiveModel {
        id_url: Set(id_url),
        content: Set(compressed.to_owned()),
        ..Default::default()
    };
    url_content_active_model.insert(&connection).await.unwrap();
}
