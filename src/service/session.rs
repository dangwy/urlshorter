use crate::client::redis::RedisClient;
use crate::error::{AppError, AppResult};
use crate::utils::claim::UserClaims;
use tracing::info;
use uuid::Uuid;

use crate::service::redis::SessionKey;

pub async fn check(redis: &RedisClient, claims: &UserClaims) -> AppResult<Uuid> {
	let session_key = SessionKey {
		u_id: claims.uid,
	};
	let session_id = crate::service::redis::get(redis, &session_key)
		.await?
		.ok_or_else(|| {
			AppError::NotFoundError(crate::error::Resource {
				details: vec![("session_key".to_string(), claims.sid.to_string())],
				resource_type: crate::error::ResourceType::Session,
			})
		})?;
	if claims.sid != session_id {
		info!("Session id invalid so deleting it: {session_key:?}.");
		crate::service::redis::del(redis, &session_key).await?;
		return Err(AppError::InvalidSessionError(
			"Session is Invalid".to_string(),
		));
	}
	Ok(claims.uid)
}

pub async fn set(redis: &RedisClient, uid: Uuid) -> AppResult<Uuid> {
	let (key, value) = generate(uid);
	crate::service::redis::set(redis, (&key, &value)).await?;
	Ok(value)
}

pub fn generate(uid: Uuid) -> (SessionKey, Uuid) {
	let session_id = Uuid::new_v4();
	let key = SessionKey { u_id: uid };
	(key, session_id)
}
