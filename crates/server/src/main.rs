#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

pub mod routes;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![
                routes::url::get_urls,
                routes::url::get_url_by_id,
                routes::url::get_by_filter_and_query,
                routes::url::url_scrap,
                routes::tag::get_tags,
                routes::tag::get_urls_by_tag_id,
            ],
        )
        .register("/", catchers![not_found])
}
