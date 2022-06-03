use common::database::get_connection;
use entity::{tag, url};
use rocket::get;
use sea_orm::{EntityTrait, JsonValue};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug)]
struct TagUrlsResponse {
    #[serde(flatten)]
    tag: tag::Model,
    urls: Vec<url::Model>,
}
#[get("/tag")]
pub async fn get_tags() -> JsonValue {
    let connection = get_connection().await.unwrap_or_default();
    let model = tag::Entity::find()
        .all(&connection)
        .await
        .unwrap_or_default();
    json!(model)
}

#[get("/tag/<id_tag>")]
pub async fn get_urls_by_tag_id(id_tag: i32) -> JsonValue {
    let connection = get_connection().await.unwrap_or_default();
    let model = tag::Entity::find_by_id(id_tag)
        .find_with_related(url::Entity)
        .all(&connection)
        .await
        .unwrap_or_default()[0]
        .to_owned();

    json!(TagUrlsResponse {
        tag: model.0,
        urls: model.1
    })
}
