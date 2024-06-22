use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
// use uuid::Uuid;

// urls response
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RedirectUrlResponse {
    pub original_url: String,
}

// tags response
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTagResponse {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TagResponse {
    pub id: i64,
    pub tag: String,
    pub domain: String,
    pub created_at: NaiveDateTime,
}