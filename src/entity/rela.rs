// use chrono::{DateTime, Utc};
use fake::Dummy;
use sea_orm::entity::prelude::*;

use crate::error::ResourceType;
use super::AppEntity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "links", table_name = "rela")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub rela_id: i64,
    pub tag_id: i64,
    pub domain: String,
    pub deleted: bool,
    pub created_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}