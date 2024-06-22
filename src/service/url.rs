use axum::Json;
use sea_orm::{ActiveModelTrait, ConnectionTrait};
use sea_orm::DatabaseTransaction;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use tracing::{error, info};
// use crate::client::redis_cluster::RedisClientExt;
use crate::server::state::AppState;
use crate::service;
use crate::service::redis::{RelaIdKey, UrlKey};
use crate::dto::request::*;
use crate::dto::response::*;
use crate::dto::*;
use crate::error::invalid_input_error;
use crate::error::AppResult;
use crate::repo;
use crate::constant::REDIS_KEY_PREFIX;
use crate::entity::urls::ActiveModel;
use chrono::prelude::*;
use serde_json::from_str;
use test_context::futures::future;
use crate::entity;
use crate::entity::urls::*;
use crate::error::ToAppResult;

use crate::utils;

pub async fn create(state: AppState, req: CreateUrlRequest) -> AppResult<UrlResponse> {
    // If alias is empty, generate a new one, otherwise set it to redis
    let mut alias = if req.alias.is_empty() {
        let key = UrlKey {
            domain: req.domain.clone()  + ":" + "urlshorter"
        };
        let incr_id = service::redis::incr(&state.redis, &key).await?;
        base62::encode(incr_id as u128)
    }else{
        req.alias.clone()
    };

    let res = UrlResponse {
        domain: req.domain.clone(),
        alias: alias.clone(),
        shorter_url: req.domain + "/" + alias.as_mut_str(),
        deleted: false,
        tags: req.tags.split(",").map(|s| s.to_string()).collect(),
        created_at: Utc::now().naive_utc(),
        expired_at: {
            if req.expired_at.is_empty() {
                None
            }else {
                NaiveDateTime::parse_from_str(req.expired_at.as_str(), "%Y-%m-%d %H:%M:%S").ok()
            }
        },
        original_url: req.original_url,
        description: req.description,
        hits: 0,
    };

    let tx = state.db.begin().await?;
    let _ = repo::urls::save(&tx, &res).await?;
    tx.commit().await?;

    Ok(res)
}

pub async fn get(state: AppState, domain: &str, alias: &str) -> AppResult<UrlResponse> {
    let urls_model = repo::urls::find_by_alias(&state.db, domain, alias).await?;
    if let Some(res) = urls_model {
        let tags = repo::tags::find_by_ids(&state.db, &domain, res.tags.unwrap_or_default()).await?;
        let url = UrlResponse {
            domain: res.domain.clone(),
            alias: res.alias,
            shorter_url: res.short_url,
            deleted: res.deleted,
            tags,
            created_at: res.created_at,
            expired_at: res.expired_at,
            original_url: res.original_url,
            description: res.description.unwrap_or_default(),
            hits: res.hits,
        };
        Ok(url)
    }else {
        Err(invalid_input_error("alias","Url not found."))
    }
}

pub async fn delete(state: AppState, domain: &str, alias: &str) -> AppResult<i64> {
    let urls_model = repo::urls::find_by_alias(&state.db, domain, alias).await?;
    if let Some(res) = urls_model {
            let tx = state.db.begin().await?;
            let _ = repo::urls::update_deleted(&tx, res.id).await?;
            tx.commit().await?;
            Ok(res.id)
    }else{
        Err(invalid_input_error("alias","Url not found."))
    }
}

pub async fn patch(state: AppState, domain: &str, alias: &str, req: PatchUrlRequest)
    -> AppResult<UrlResponse> {
    let urls_model = repo::urls::find_by_alias(&state.db, domain, alias).await?;

    let tx = state.db.begin().await?;

    let ret = if let Some(res) = urls_model {
        let original_url = if req.original_url.is_empty(){
            res.original_url
        }else {
            req.original_url
        };

        let expired_at = if req.expired_at.is_empty() {
            res.expired_at
        }else{
            NaiveDateTime::parse_from_str(req.expired_at.as_str(), "%Y-%m-%d %H:%M:%S").ok()
        };

        let tags = if req.tags.is_empty() {
            repo::tags::find_by_ids(&state.db, &domain, res.tags.unwrap()).await?
        }else{
            req.tags.split(",").map(|s| s.to_string()).collect()
        };
        let tag_ids = repo::tags::get_or_save_by_tags(&tx, &domain, &tags).await?;

        let model = repo::urls::update_some(&tx, res.id,&original_url, tag_ids, expired_at).await?;
        let url = UrlResponse{
            domain: model.domain.clone(),
            alias: model.alias.clone(),
            shorter_url: model.short_url.clone(),
            deleted: model.deleted,
            tags,
            created_at: model.created_at,
            expired_at: model.expired_at,
            original_url: model.original_url,
            description: res.description.unwrap_or_default(),
            hits: res.hits,
        };

        tx.commit().await?;
        Some(url)
    }else{
        None
    };

    Ok(ret.unwrap())
}

pub async fn redirect(state: AppState, domain: &str, alias: &str) -> AppResult<RedirectUrlResponse> {
    let urls_model = repo::urls::find_by_alias(&state.db, domain, alias).await?;
    if let Some(res) = urls_model {
        let url = RedirectUrlResponse {
            original_url: res.original_url,
        };

        let new_hits = res.hits + 1;
        let tx = state.db.begin().await?;
        let _ = repo::urls::update_hits(&tx, res.id, new_hits).await?;
        tx.commit().await?;

        Ok(url)
    }else{
        Err(invalid_input_error("alias","Url not found."))
    }
}