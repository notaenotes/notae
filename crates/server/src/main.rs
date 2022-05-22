#[macro_use]
extern crate rocket;

use common::database::get_connection;
use entity::url::Entity as Url;
use rocket::serde::json::Json;
use sea_orm::EntityTrait;
use sea_orm::JsonValue;

#[get("/")]
async fn hello() -> Json<Vec<JsonValue>> {
    let connection = get_connection().await;
    Json(Url::find().into_json().all(&connection).await.unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
