use sea_orm::ActiveValue::Set;
use sea_orm::{
     ActiveModelTrait, ColumnTrait, DatabaseConnection,
    DatabaseTransaction, EntityTrait,  QueryFilter
};
// use serde::de::Unexpected::Option;
use crate::entities::tags::Model;
use crate::{
	entities::tags,
	error::AppResult,
};

#[tracing::instrument(skip_all)]
pub async fn save(tx: &DatabaseTransaction, domain: &str, tag: &str) -> AppResult<i64> {
    let model = tags::ActiveModel {
        tag: Set(tag.to_string()),
        created_at: Set(chrono::Local::now().naive_utc()),
        domain: Set(domain.to_string()),
        ..Default::default()
    }
    .insert(tx)
    .await?;

    Ok(model.id)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_id(conn: &DatabaseConnection, domain: &str, tag_id: i64,
) -> AppResult<Option<tags::Model>> {
    let model = tags::Entity::find_by_id(tag_id)
        .filter(tags::Column::Domain.eq(domain))
        .one(conn)
        .await?;
    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_ids(conn: &DatabaseConnection, domain: &str, tag_ids: Vec<i64>,
) -> AppResult<Vec<String>> {
    let mut tags: Vec<String> = vec![];
    let model = tags::Entity::find()
        .filter(
            tags::Column::Domain
                .eq(domain)
                .and(tags::Column::Id.is_in(tag_ids)),
        )
        .all(conn)
        .await?;
    model.iter().for_each(|m| tags.push(m.tag.clone()));
    Ok(tags)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_tag(conn: &DatabaseConnection, domain: &str, tag: &str,
) -> AppResult<Option<Model>> {
    let model = tags::Entity::find()
        .filter(
            tags::Column::Tag
                .eq(tag)
                .and(tags::Column::Domain.eq(domain)),
        )
        .one(conn)
        .await?;
    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_tag_tx(tx: &DatabaseTransaction, domain: &str, tag: &str,
) -> AppResult<Option<Model>> {
    let model = tags::Entity::find()
        .filter(
            tags::Column::Tag
                .eq(tag)
                .and(tags::Column::Domain.eq(domain)),
        )
        .one(tx)
        .await?;
    Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn get_or_save_by_tags(tx: &DatabaseTransaction, domain: &str, tags: &Vec<String>,
) -> AppResult<Vec<i64>> {
    let mut tag_ids: Vec<i64> = vec![];

    for tag in tags {
        let model = find_by_tag_tx(tx, domain, tag).await?;
        if model.is_none() {
            let tag_id = save(tx, domain, tag).await?;
            tag_ids.push(tag_id);
        } else {
            tag_ids.push(model.unwrap().id);
        }
    }

    Ok(tag_ids)
}

#[tracing::instrument(skip_all)]
pub async fn delete_by_tag(tx: &DatabaseTransaction, domain: &str, tag: &str) -> AppResult<()> {
    let _ = tags::Entity::delete_many()
        .filter(
            tags::Column::Domain
                .eq(domain)
                .and(tags::Column::Tag.eq(tag)),
        )
        .exec(tx)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::TransactionTestContext;
    use test_context::test_context;

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
