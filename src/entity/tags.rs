// use chrono::{DateTime, Utc};
use fake::Dummy;
use sea_orm::entity::prelude::*;

use crate::error::ResourceType;
use super::AppEntity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "links", table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub tag: String,
    pub domain: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Dummy, DeriveEntityModel)]
// #[sea_orm(table_name = "tags")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i64,
//     pub tag: String,
//     pub deleted: bool,
//     pub created_at: DateTime<Utc>,
//     pub description: String,
// }
//
// impl super::AppEntity for Model {
//     const RESOURCE: crate::error::ResourceType = ResourceType::Tags;
// }
//
// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(has_many = "super::tags::Entity")]
//     Tags,
// }
//
// impl Related<super::rela::Entity> for crate::entity::urls::Entity {
//     fn to() -> RelationDef {
//         crate::entity::urls::Relation::Rela.def()
//     }
// }