use log::info;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
    DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait
};
use sea_orm::ActiveValue::Set;
// use serde::de::Unexpected::Option;
use crate::{
    dto::{request::CreateUrlRequest, response::UrlResponse}, // Direction, PageQueryParam
    entity,
    error::{AppResult, ToAppResult},
    utils,
};


#[tracing::instrument(skip_all)]
pub async fn save(tx: &DatabaseTransaction, domain: &str, tag: &str,
) -> AppResult<i64> {
    let model = entity::tags::ActiveModel{
        tag: Set(tag.to_string()),
        deleted: Set(false),
        created_at: Set(chrono::Local::now().naive_utc()),
        domain: Set(domain.to_string()),
        ..Default::default()
    }.insert(tx).await?;

    Ok(model.id)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_id(conn: &DatabaseConnection, tag_id: i64, domain: &str
) -> AppResult<Option<entity::tags::Model>> {
    let model = entity::tags::Entity::find_by_id(tag_id)
        .filter(entity::urls::Column::Deleted.eq(false)
            .and(entity::urls::Column::Domain.eq(domain)))
        .one(conn)
        .await?;
    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_ids(conn: &DatabaseConnection, tag_ids: Vec<i64>, domain: &str
) -> AppResult<Vec<String>> {
    let mut tags: Vec<String> = vec![];
    let model = entity::tags::Entity::find()
        .filter(entity::tags::Column::Deleted.eq(false)
            .and(entity::tags::Column::Domain.eq(domain))
            .and(entity::tags::Column::Id.is_in(tag_ids))
        )
        .all(conn)
        .await?;
    model.iter().for_each(|m| tags.push(m.tag.clone()));
    Ok(tags)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_tag(tx: &DatabaseTransaction, tag: &str, domain: &str
) -> AppResult<Option<i64>> {
    let model = entity::tags::Entity::find()
        .filter(entity::tags::Column::Tag.eq(tag)
            .and(entity::tags::Column::Deleted.eq(false))
            .and(entity::tags::Column::Domain.eq(domain)))
        .one(tx)
        .await?;
    Ok(model.map(|m| m.id))
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_context::test_context;
    use crate::entity::TransactionTestContext;

    #[test_context(TransactionTestContext)]
    #[tokio::test]
    async fn test_find_by_tag(ctx: &mut TransactionTestContext) {
        let tag = "tag1";
        let domain = "test.com";
        let tag_id = find_by_tag(&**ctx, tag, domain).await.unwrap();

    }
// let tags: tags::Model = tags.insert(&db).await?;
//
// println!("Tags created with ID: {}, TAG: {}", tags.id, tags.tag);

}