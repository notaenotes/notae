use sea_orm::{entity::prelude::*, ActiveValue::Set, SelectTwoMany};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "urls")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique)]
    pub url: String,

    #[serde(skip_serializing)]
    #[sea_orm(unique)]
    hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UrlContent,
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::url_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::url_tag::Relation::Url.def().rev())
    }
}

impl Related<super::url_content::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UrlContent.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UrlContent => Entity::has_many(super::url_content::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        if Url::parse(self.url.as_ref()).is_err() {
            Err(DbErr::Custom(format!("Is not a valid URL {}", insert)))
        } else {
            self.hash = Set(format!("{:x}", md5::compute(self.url.as_ref())));
            Ok(self)
        }
    }
}

impl Entity {
    pub fn find_by_hash(hash: &str) -> Select<Entity> {
        self::Entity::find().filter(self::Column::Hash.eq(hash))
    }

    pub fn find_by_id_with_related_tags(
        id: i32,
    ) -> SelectTwoMany<self::Entity, super::tag::Entity> {
        self::Entity::find_by_id(id).find_with_related(super::tag::Entity)
    }
}
