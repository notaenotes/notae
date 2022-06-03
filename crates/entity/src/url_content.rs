use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "urls_content")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub id_url: i32,
    #[sea_orm(unique)]
    pub hash: String,
    pub content: Vec<u8>,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Url,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Url => Entity::belongs_to(super::url::Entity)
                .from(Column::IdUrl)
                .to(super::url::Column::Id)
                .into(),
        }
    }
}
impl Related<super::url::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Url.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.hash = Set(format!("{:x}", md5::compute(self.content.as_ref())));
        self.created_at = Set(chrono::Utc::now());
        Ok(self)
    }
}
