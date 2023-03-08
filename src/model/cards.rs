//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "cards")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub card: Uuid,
    pub name: Option<String>,
    pub phoneme: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub image: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::questions::Entity")]
    Questions,
}

impl Related<super::questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Questions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
