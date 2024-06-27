use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::constant::BEARER;

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

// service status response
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ServiceStatusResponse {
    pub db: bool,
    pub redis: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}

// Token
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_in: u64,
}

impl TokenResponse {
    pub fn new(access_token: String, refresh_token: String, expire_in: u64) -> Self {
        Self {
            token_type: BEARER.to_string(),
            access_token,
            refresh_token,
            expire_in,
        }
    }
    
    pub fn default() -> Self {
        Self {
            token_type: BEARER.to_string(),
            access_token: "".to_string(),
            refresh_token: "".to_string(),
            expire_in: 0,
        }
    }
}