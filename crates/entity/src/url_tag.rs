use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "urls_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_url: i32,
    #[sea_orm(primary_key)]
    pub id_tag: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Url,
    Tag,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Url => Entity::belongs_to(super::url::Entity)
                .from(Column::IdUrl)
                .to(super::url::Column::Id)
                .into(),
            Self::Tag => Entity::belongs_to(super::tag::Entity)
                .from(Column::IdTag)
                .to(super::tag::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
