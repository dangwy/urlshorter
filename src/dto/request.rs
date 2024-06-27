// use fake::faker::internet::en::{Password, SafeEmail, Username};
use fake::{faker, Dummy};
use garde::Validate;
use lettre::transport::smtp::extension::ClientId::Domain;
use serde::{Deserialize, Serialize};
// use strum::Display;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, Validate, utoipa::ToSchema)]
pub struct CreateUrlRequest {
    #[garde(skip)]
    pub original_url: String,
    #[garde(skip)]
    pub domain: String,
    #[garde(skip)]
    pub alias: String,
    #[garde(skip)]
    pub tags: String,
    #[garde(skip)]
    pub expired_at: String,
    #[garde(skip)]
    pub description: String,
}

impl CreateUrlRequest {
    pub fn new(
        url: &str,
        domain: &str,
        alias: &str,
        tags: &str,
        expired_at: &str,
        desc: &str,
    ) -> Self {
        Self {
            original_url: url.to_string(),
            domain: domain.to_string(),
            alias: alias.to_string(),
            tags: tags.to_string(),
            expired_at: expired_at.to_string(),
            description: desc.to_string(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Debug, Deserialize, ToSchema, Validate, IntoParams)]
pub struct GetUrlQueryParam {
    #[garde(skip)]
    pub domain: String,
    #[garde(skip)]
    pub alias: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate, IntoParams)]
pub struct PatchUrlRequest {
    #[garde(skip)]
    pub original_url: String,
    #[garde(skip)]
    pub tags: String,
    #[garde(skip)]
    pub expired_at: String,
}

// tags request
#[derive(Debug, Deserialize, Serialize, Validate, utoipa::ToSchema)]
pub struct CreateTagRequest {
    #[garde(skip)]
    pub tag: String,
    #[garde(skip)]
    pub domain: String,
    #[garde(skip)]
    pub expired_at: String,
}

// Token
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct RefreshTokenRequest {
    #[garde(length(min = 30))]
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct GenerateTokenRequest {
    #[garde(length(min = 10))]
    pub access_key: String,
    #[garde(length(min = 10))]
    pub secret_key: String,
}