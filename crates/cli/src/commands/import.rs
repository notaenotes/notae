use crate::import;
use crate::import::getpocket::NewUrl;
use common::database;
use entity::{prelude::*, tag};
use sea_orm::{entity::Set, ColumnTrait, EntityTrait, QueryFilter};
use std::path::PathBuf;

pub async fn init(file_path: &Option<PathBuf>) {
    let captured_links = import::getpocket::get_links(&file_path.to_owned().unwrap());
    insert_captured_links(captured_links).await;
}

async fn insert_captured_links(captured_links: Vec<NewUrl>) {
    let connection = database::get_connection().await;

    for link in captured_links {
        let url = UrlActiveModel {
            url: Set(link.url),
            ..Default::default()
        };
        let url_insert_res = Url::insert(url).exec(&connection).await.unwrap();

        for link_tag in link.tags {
            if !link_tag.is_empty() {
                let tag_exists_res = Tag::find()
                    .filter(tag::Column::Tag.eq(link_tag.to_owned()))
                    .one(&connection)
                    .await
                    .unwrap();

                let tag_id: i32 = if !tag_exists_res.to_owned().is_some() {
                    let tag = TagActiveModel {
                        tag: Set(link_tag.to_owned()),
                        ..Default::default()
                    };

                    Tag::insert(tag)
                        .exec(&connection)
                        .await
                        .unwrap()
                        .last_insert_id as i32
                } else {
                    tag_exists_res.unwrap().id
                };

                let url_tag = UrlTagActiveModel {
                    id_url: Set(url_insert_res.last_insert_id as i32),
                    id_tag: Set(tag_id),
                };
                UrlTag::insert(url_tag).exec(&connection).await.unwrap();
            }
        }
    }
}
