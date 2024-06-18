use chrono::Utc;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
    DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTime;
use crate::{entity, repo};
use crate::error::AppResult;
use chrono::prelude::*;

pub async fn save_by_tagid(tx: &DatabaseTransaction,rela_id: i64, tag_id: i64, domain: &str,
) -> AppResult<i64> {
    let model = entity::rela::ActiveModel{
        rela_id: Set(rela_id),
        tag_id: Set(tag_id),
        domain: Set(domain.to_string()),
        deleted: Set(false),
        created_at: Set(Utc::now().naive_utc()),
        deleted_at: Set(None),
        ..Default::default()
    }.insert(tx).await?;

    Ok(model.id)
}

#[tracing::instrument(skip_all)]
pub async fn save_by_tags(tx: &DatabaseTransaction, rela_id: i64, tags: &Vec<String>, domain: &str,
) -> AppResult<()> {
    let mut tag_ids: Vec<i64> = vec![];

    // if tags table exists, get id, if not, create a new one
    for tag in tags {
        let tag_id = repo::tags::find_by_tag(tx, &tag, &domain).await?;
        match tag_id {
            Some(id) => {
                tag_ids.push(id);
            },
            None => {
                let id = repo::tags::save(tx, domain, tag).await?;
                tag_ids.push(id);
            }
        }
    }

    for tag_id in tag_ids {
        let _ = save_by_tagid(tx, rela_id, tag_id, domain).await?;
    }

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn find_by_relaid(conn: &DatabaseConnection, rela_id: i64, domain: &str,
) -> AppResult<Vec<String>> {
    let tag_ids = entity::rela::Entity::find()
        .filter(entity::rela::Column::Deleted.eq(false)
            .and(entity::rela::Column::Domain.eq(domain))
            .and(entity::rela::Column::RelaId.eq(rela_id))
        )
        .all(conn)
        .await?.iter().map(|m| m.tag_id).collect();
    Ok(repo::tags::find_by_ids(conn, tag_ids, &domain).await?)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_tagid(tx: &DatabaseTransaction, tag_id: i64,
) -> AppResult<Option<i64>> {
    let model = entity::rela::Entity::find()
        .filter(entity::rela::Column::Deleted.eq(false)
            .and(entity::rela::Column::TagId.eq(tag_id)))
        .one(tx)
        .await?;
    Ok(model.map(|m| m.id))
}

/*
#[tracing::instrument(skip_all)]
pub async fn get_by_tags(tx: &DatabaseTransaction, tags: &Vec<String>, domain: &str)
    -> AppResult<i64> {
    let mut rela_id: i64 = 0;
    let mut tag_ids: Vec<i64> = vec![];

    // if tags table exists, get id, if not, create a new one
    for tag in tags {
        let tag_id = repo::tags::find_by_tag(tx, &tag, &domain).await?;
        match tag_id {
            Some(id) => {
                tag_ids.push(id);
            },
            None => {
                let id = repo::tags::save(tx, domain, tag).await?;
                tag_ids.push(id);
            }
        }
    }

    // let id = find_by_tagid(tx, id).await?;
    // match id {
    //     Some(id) => {
    //         rela_id = save(tx, id).await?;
    //     },
    //     None => {
    //         let tid = repo::tags::save(tx, domain, tag).await?;
    //         rela_id = save(tx, tid).await?;
    //     }
    // }

        // let model = entity::rela::Entity::find()
        //     .filter(entity::rela::Column::TagId.eq(tag))
        //     .filter(entity::urls::Column::Deleted.eq(false))
        //     .one(conn)
        //     .await?;
        // if model.is_some() {
        //     return Ok(Some(model.unwrap().id));
        // }
    // }

    return Ok(rela_id)
}
*/
