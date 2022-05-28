#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};
mod routes;

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
            routes![routes::url::get_all_urls, routes::url::get_url_by_id],
        )
        .register("/", catchers![not_found])
}
