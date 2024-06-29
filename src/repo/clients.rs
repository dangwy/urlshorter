use sea_orm::ActiveValue::Set;
use sea_orm::{
	sea_query::Expr, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
	DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait,
};
// use serde::de::Unexpected::Option;

use crate::{
	entities::clients,
	error::{AppResult, ToAppResult},
};
use uuid::Uuid;

#[tracing::instrument(skip_all)]
pub async fn find_by_id<C>(conn: &C, id: Uuid) -> AppResult<Option<clients::Model>>
where
	C: ConnectionTrait,
{
	let model = clients::Entity::find_by_id(id)
		.filter(clients::Column::IsActive.eq(true)).one(conn).await?;
	Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_ak_and_sk<C>(conn: &C, ak: &str, sk: &str) -> AppResult<Option<clients::Model>>
where
	C: ConnectionTrait,
{
	let model = clients::Entity::find().
		filter(clients::Column::AccessKey.eq(ak)
			.and(clients::Column::SecretKey.eq(sk))
			.and(clients::Column::IsActive.eq(true))).one(conn).await?;
	Ok(model)
}