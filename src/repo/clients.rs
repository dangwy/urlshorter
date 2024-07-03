use sea_orm::{ColumnTrait,  ConnectionTrait, EntityTrait, QueryFilter};

use crate::{
	entities::clients,
	error::AppResult,
};

#[tracing::instrument(skip_all)]
pub async fn find_by_id<C>(conn: &C, id: i64) -> AppResult<Option<clients::Model>>
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