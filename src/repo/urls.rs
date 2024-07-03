use crate::dto::response::UrlResponse;
use crate::{
	entities,
	error::AppResult,
	repo,
};
use chrono::{NaiveDateTime};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait,   DatabaseConnection,
    DatabaseTransaction,  EntityTrait,  QueryFilter,
};

#[tracing::instrument]
pub async fn save(tx: &DatabaseTransaction, res: &UrlResponse, client_id: i64) -> AppResult<i64> {
    let tag_ids = repo::tags::get_or_save_by_tags(tx, &res.domain, &res.tags).await?;

    let url = entities::urls::ActiveModel {
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
        client_id: Set(client_id),
        ..Default::default()
    }
    .insert(tx)
    .await?;

    Ok(url.id)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_alias(
    conn: &DatabaseConnection,
    client_id: i64,
    domain: &str,
    alias: &str,
) -> AppResult<Option<entities::urls::Model>> {
    let model = entities::urls::Entity::find()
        .filter(
	        entities::urls::Column::ClientId.eq(client_id)
                .and(entities::urls::Column::Domain.eq(domain))
                .and(entities::urls::Column::Alias.eq(alias))
                .and(entities::urls::Column::Deleted.eq(false)),
        )
        .one(conn)
        .await?;
    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn delete(conn: &DatabaseTransaction, id: i64) -> AppResult<()> {
    let url = entities::urls::ActiveModel {
        id: Set(id),
        ..Default::default()
    };

    let _ = entities::urls::Entity::delete(url).exec(conn).await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn update_deleted(conn: &DatabaseTransaction, id: i64) -> AppResult {
    let url = entities::urls::ActiveModel {
        id: Set(id),
        deleted: Set(true),
        ..Default::default()
    };
    entities::urls::Entity::update(url).exec(conn).await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn update_hits(
    conn: &DatabaseTransaction,
    id: i64,
    hits: i32,
) -> AppResult<entities::urls::Model> {
    let url = entities::urls::ActiveModel {
        id: Set(id),
        hits: Set(hits),
        ..Default::default()
    };
    let model = entities::urls::Entity::update(url).exec(conn).await?;

    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn update_some(
    conn: &DatabaseTransaction,
    id: i64,
    original_url: &str,
    tags: Vec<i64>,
    expired_at: Option<NaiveDateTime>,
) -> AppResult<entities::urls::Model> {
    let url = entities::urls::ActiveModel {
        id: Set(id),
        original_url: Set(original_url.to_string()),
        tags: Set(Some(tags)),
        expired_at: Set(expired_at),
        ..Default::default()
    };
    let model = entities::urls::Entity::update(url).exec(conn).await?;

    Ok(model)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use test_context::test_context;
    // use crate::entities::TransactionTestContext;
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
