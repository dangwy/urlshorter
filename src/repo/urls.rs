use chrono::{NaiveDateTime, Utc};
use sea_orm::prelude::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
    DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, DbBackend,
};
use serde_json::from_str;
use uuid::Uuid;
use crate::dto::{request::CreateUrlRequest, response::UrlResponse};
use crate::{entity, error::{AppResult, ToAppResult}, repo, utils};

#[tracing::instrument]
pub async fn save(tx: &DatabaseTransaction, res: &UrlResponse, rela_id: i64) -> AppResult<i64> {
    let link = entity::urls::ActiveModel {
        domain: Set(res.domain.clone()),
        alias: Set(res.alias.clone()),
        short_url: Set(res.shorter_url.clone()),
        deleted: Set(res.deleted),
        rela_id: Set(Some(rela_id)),
        created_at: Set(res.created_at),
        expired_at: Set(res.expired_at),
        original_url: Set(res.original_url.clone()),
        description: Set(Some(res.description.clone())),
        hits: Set(res.hits),
        ..Default::default()
    }
    .insert(tx)
    .await?;

    if !res.tags.is_empty() {
        repo::rela::save_by_tags(tx, rela_id, &res.tags, &res.domain,).await?;
    }

    Ok(link.id)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_domain_and_alias<C>(conn: &C, domain: &str, alias: &str,
) -> AppResult<Option<entity::urls::Model>>
where
    C: ConnectionTrait,
{
    let model = entity::urls::Entity::find()
        .filter(
            entity::urls::Column::Domain.eq(domain)
                .and(entity::urls::Column::Alias.eq(alias))
                .and(entity::urls::Column::Deleted.eq(false)),
        )
        .one(conn)
        .await?;
    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_domain_and_alias_1(conn: &DatabaseTransaction, domain: &str, alias: &str,
) -> AppResult<Option<entity::urls::Model>>
{
    let model = entity::urls::Entity::find()
        .filter(
            entity::urls::Column::Domain.eq(domain)
                .and(entity::urls::Column::Alias.eq(alias))
                .and(entity::urls::Column::Deleted.eq(false)),
        )
        .one(conn)
        .await?;
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
