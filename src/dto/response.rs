use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
// use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UrlResponse {
    pub domain: String,
    pub alias: String,
    pub shorter_url: String,
    pub deleted: bool,
    pub tags: Vec<String>,
    pub created_at: NaiveDateTime,
    pub expired_at: Option<NaiveDateTime>,
    pub original_url: String,
    pub description: String,
    pub hits: i32,
}
