use crate::dto::{request::CreateUrlRequest, response::UrlResponse};
use crate::{
    entity,
    error::{AppResult, ToAppResult},
    repo, utils,
};
use chrono::{NaiveDateTime, Utc};
use sea_orm::prelude::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
    DatabaseTransaction, DbBackend, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    SelectColumns,
};
use serde_json::from_str;
use uuid::Uuid;

#[tracing::instrument]
pub async fn save(tx: &DatabaseTransaction, res: &UrlResponse) -> AppResult<i64> {
    let tag_ids = repo::tags::get_or_save_by_tags(tx, &res.domain, &res.tags).await?;

    let url = entity::urls::ActiveModel {
        domain: Set(res.domain.clone()),
        alias: Set(res.alias.clone()),
        short_url: Set(res.shorter_url.clone()),
        deleted: Set(res.deleted),
        tags: Set(Some(tag_ids)),
        created_at: Set(res.created_at),
        expired_at: Set(res.expired_at),
        original_url: Set(res.original_url.clone()),
        description: Set(Some(res.description.clone())),
        hits: Set(res.hits),
        ..Default::default()
    }
    .insert(tx)
    .await?;

    Ok(url.id)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_alias(
    conn: &DatabaseConnection,
    domain: &str,
    alias: &str,
) -> AppResult<Option<entity::urls::Model>> {
    let model = entity::urls::Entity::find()
        .filter(
            entity::urls::Column::Domain
                .eq(domain)
                .and(entity::urls::Column::Alias.eq(alias))
                .and(entity::urls::Column::Deleted.eq(false)),
        )
        .one(conn)
        .await?;
    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn delete(conn: &DatabaseTransaction, id: i64) -> AppResult<()> {
    let url = entity::urls::ActiveModel {
        id: Set(id),
        ..Default::default()
    };

    let _ = entity::urls::Entity::delete(url).exec(conn).await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn update_deleted(conn: &DatabaseTransaction, id: i64) -> AppResult {
    let url = entity::urls::ActiveModel {
        id: Set(id),
        deleted: Set(true),
        ..Default::default()
    };
    entity::urls::Entity::update(url).exec(conn).await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn update_hits(
    conn: &DatabaseTransaction,
    id: i64,
    hits: i32,
) -> AppResult<entity::urls::Model> {
    let url = entity::urls::ActiveModel {
        id: Set(id),
        hits: Set(hits),
        ..Default::default()
    };
    let model = entity::urls::Entity::update(url).exec(conn).await?;

    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn update_some(
    conn: &DatabaseTransaction,
    id: i64,
    original_url: &str,
    tags: Vec<i64>,
    expired_at: Option<NaiveDateTime>,
) -> AppResult<entity::urls::Model> {
    let url = entity::urls::ActiveModel {
        id: Set(id),
        original_url: Set(original_url.to_string()),
        tags: Set(Some(tags)),
        expired_at: Set(expired_at),
        ..Default::default()
    };
    let model = entity::urls::Entity::update(url).exec(conn).await?;

    Ok(model)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use test_context::test_context;
    // use crate::entity::TransactionTestContext;
    //
    // // #[test_context(TransactionTestContext)]
    // #[tokio::test]
    // async fn test_find_by_domain_and_alias() {
    //     let domain = "baidu.com";
    //     let alias = "123456";
    //     let model = find_by_domain_and_alias(ctx: &mut TransactionTestContext, domain, alias).await?;
    //     println!("model: {:?}", model)
    // }
}
