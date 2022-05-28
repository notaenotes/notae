use common::database::get_connection;
use entity::url;
use rocket::serde::json::{json, Value};
use sea_orm::EntityTrait;
use sea_orm::JsonValue;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct UrlTagResponse {
    #[serde(flatten)]
    url: entity::url::Model,
    tags: Vec<entity::tag::Model>,
}

#[get("/url")]
pub async fn get_all_urls() -> JsonValue {
    let connection = get_connection().await.unwrap_or_default();
    let model = url::Entity::find()
        .all(&connection)
        .await
        .unwrap_or_default()
        .to_owned();
    json!(model)
}

#[get("/url/<id_url>")]
pub async fn get_url_by_id(id_url: i32) -> Value {
    let connection = get_connection().await.unwrap_or_default();
    let model = url::Entity::find_by_id(id_url)
        .find_with_related(entity::tag::Entity)
        .all(&connection)
        .await
        .unwrap_or_default()[0]
        .to_owned();

    json!(UrlTagResponse {
        url: model.0,
        tags: model.1,
    })
}
