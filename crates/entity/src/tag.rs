use sea_orm::{entity::prelude::*, ActiveValue::Set, SelectTwoMany};
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique)]
    pub tag: String,

    #[serde(skip_serializing)]
    #[sea_orm(unique)]
    hash: String,

    #[sea_orm(unique)]
    slug: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl Related<super::url::Entity> for Entity {
    fn to() -> RelationDef {
        super::url_tag::Relation::Url.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::url_tag::Relation::Tag.def().rev())
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.tag = Set(String::from(self.tag.as_ref().trim()));
        self.hash = Set(format!(
            "{:x}",
            md5::compute(self.tag.as_ref().to_lowercase().trim())
        ));
        self.slug = Set(slugify(self.tag.as_ref()));

        Ok(self)
    }
}

impl Entity {
    pub fn find_by_hash(hash: &str) -> Select<Entity> {
        self::Entity::find().filter(self::Column::Hash.eq(hash))
    }

    pub fn find_by_id_with_related_urls(
        id: i32,
    ) -> SelectTwoMany<self::Entity, super::url::Entity> {
        self::Entity::find_by_id(id).find_with_related(super::url::Entity)
    }
}
