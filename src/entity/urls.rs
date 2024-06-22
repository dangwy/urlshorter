// use chrono::{DateTime, Utc};
use fake::Dummy;
use sea_orm::entity::prelude::*;

use crate::error::ResourceType;
use super::AppEntity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "links", table_name = "urls")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub domain: String,
    #[sea_orm(column_type = "Text")]
    pub alias: String,
    #[sea_orm(column_type = "Text")]
    pub short_url: String,
    pub deleted: bool,
    pub tags: Option<Vec<i64>>,
    pub created_at: DateTime,
    pub expired_at: Option<DateTime>,
    #[sea_orm(column_type = "Text")]
    pub original_url: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub hits: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Dummy, DeriveEntityModel)]
// #[sea_orm(table_name = "urls")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i64,
//     pub domain: String,
//     pub alias: String,
//     pub shorter_url: String,
//     pub deleted: bool,
//     pub rela_id: i64,
//     pub created_at: DateTime<Utc>,
//     pub expires_at: DateTime<Utc>,
//     pub url: String,
//     pub description: String,
//     pub hits : i32,
// }
//
// impl AppEntity for Model {
//     const RESOURCE: ResourceType = ResourceType::Urls;
// }
//
// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(has_many = "super::rela::Entity")]
//     Rela,
// }
//
// impl Related<super::rela::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Rela.def()
//     }
// }

// #[async_trait::async_trait]
// impl ActiveModelBehavior for ActiveModel {}
