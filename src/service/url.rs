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

    // Get max rela_id from redis
    let key = RelaIdKey {
        domain: req.domain.clone() + ":" + "rela_id",
    };
    let max_rela_id = service::redis::incr(&state.redis, &key).await?;

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
    let sid = repo::urls::save(&tx, &res, max_rela_id).await?;
    tx.commit().await?;

    Ok(res)
}

// req: GetUrlQueryParam
pub async fn get(state: AppState, domain: &str, alias: &str ) -> AppResult<UrlResponse> {
    let tx = state.db.begin().await?;
    let res = repo::urls::find_by_domain_and_alias_1(&tx, domain, alias).await?;
    match res {
        None => {
            let e = invalid_input_error("","url not found");
            Err(e)
        }
        Some(res) => {
            let res = UrlResponse {
                domain: res.domain.clone(),
                alias: res.alias,
                shorter_url: res.short_url,
                deleted: res.deleted,
                tags: {
                    if let Some(rela_id) = res.rela_id {
                        repo::rela::find_by_relaid(&state.db, rela_id, &res.domain).await?
                    }else {
                        vec![]
                    }
                },
                created_at: res.created_at,
                expired_at: res.expired_at,
                original_url: res.original_url,
                description: res.description.unwrap_or_default(),
                hits: res.hits,
            };
            tx.commit().await?;
            Ok(res)
        }
    }

}

// UrlResponse {
//     domain: res.domain,
//     alias: res.alias,
//     shorter_url: res.short_url,
//     deleted: res.deleted,
//     tags: {
//         res.rela_id
//     },
//     created_at: res.created_at,
//     expired_at: res.expired_at,
//     original_url: res.original_url,
//     description: res.description?,
//     hits: res.hits,
// }.to_result();

/*
pub async fn delete() -> AppResult<UrlResponse> {
    info!("Get a user from alias request: {req:?}.");
}

pub async fn direct() -> AppResult<UrlResponse> {
    info!("Get a user from alias request: {req:?}.");
}

 */
