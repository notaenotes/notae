use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
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
pub enum Relation {}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::url_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::url_tag::Relation::Url.def().rev())
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        if !Url::parse(self.url.as_ref()).is_ok() {
            Err(DbErr::Custom(format!("asdas {}", insert)))
        } else {
            self.hash = ActiveValue::Set(format!("{:x}", md5::compute(self.url.as_ref())));
            Ok(self)
        }
    }
}
