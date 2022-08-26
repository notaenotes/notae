use sea_orm::EntityTrait;
use url::Url;

use common::database::get_connection;

pub async fn init(url_identitier: &i32) {
    let url = get_url_model(url_identitier).await.url;
    let parsed_url = Url::parse(&url).unwrap();
    let domain = parsed_url.host().unwrap().to_string();
    let root_domain = Vec::from_iter(domain.split(".").map(String::from));
    println!("{:#?}", root_domain);
    // dbg!(&domain);

    // dbg!(Url::parse(url));
    // let html = get_url_html(url).await;
    // dbg!(html);
}

async fn get_url_html(url: String) -> String {
    reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_default()
}

async fn get_url_model(url_identitier: &i32) -> entity::url::Model {
    let connection = get_connection().await.unwrap_or_default();

    entity::url::Entity::find_by_id(url_identitier.to_owned())
        .one(&connection)
        .await
        .unwrap()
        .unwrap()
}
