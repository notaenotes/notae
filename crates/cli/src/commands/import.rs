use crate::import;
use crate::import::getpocket::NewUrl;
use common::database;
use entity::{tag, url, url_tag};
use sea_orm::{entity::Set, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use std::path::PathBuf;

pub async fn init(file_path: &Option<PathBuf>) {
    let captured_links = import::getpocket::get_links(&file_path.to_owned().unwrap());
    insert_captured_links(captured_links).await;
}

async fn insert_captured_links(captured_links: Vec<NewUrl>) {
    let connection = database::get_connection().await.unwrap_or_default();

    for link in captured_links {
        let url_hash = format!("{:x}", md5::compute(&link.url));
        let link_exists_res = url::Entity::find()
            .filter(url::Column::Hash.eq(url_hash))
            .one(&connection)
            .await
            .unwrap_or_default();

        if link_exists_res.is_none() {
            let url_active_model = url::ActiveModel {
                url: Set(link.url),
                ..Default::default()
            };

            let url_active_model_res = url_active_model.insert(&connection).await;
            let url_id = url_active_model_res.unwrap().id;

            for link_tag in link.tags {
                if !link_tag.is_empty() {
                    let tag_exists_res = tag::Entity::find()
                        .filter(tag::Column::Tag.eq(link_tag.to_owned()))
                        .one(&connection)
                        .await
                        .unwrap_or_default();

                    let tag_id: i32 = if tag_exists_res.to_owned().is_none() {
                        let tag_active_model = tag::ActiveModel {
                            tag: Set(link_tag.to_owned()),
                            ..Default::default()
                        };
                        let url_active_model_res = tag_active_model.insert(&connection).await;
                        url_active_model_res.unwrap().id
                    } else {
                        tag_exists_res.unwrap().id
                    };

                    let url_tag_active_model = url_tag::ActiveModel {
                        id_url: Set(url_id),
                        id_tag: Set(tag_id),
                    };
                    let _url_tag_active_model_res = url_tag_active_model.insert(&connection).await;
                }
            }
        }
    }
}
