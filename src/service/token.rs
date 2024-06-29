use serde_json::from_str;
use crate::constant::*;
use crate::dto::response::TokenResponse;
use crate::dto::{GenerateTokenRequest, RefreshTokenRequest};
use crate::error::{AppError, AppResult, ToAppResult};
use crate::server::state::AppState;
use crate::{repo, service};
use crate::utils::claim::UserClaims;
use tracing::info;
use uuid::Uuid;

pub async fn refresh(state: &AppState, req: RefreshTokenRequest) -> AppResult<TokenResponse> {
	let user_claims = UserClaims::decode(&req.token, &REFRESH_TOKEN_DECODE_KEY)?.claims;
	info!("Refresh token: {user_claims:?}");
	// let u_id = service::session::check(&state.redis, &user_claims).await?;
	let client = crate::repo::clients::find_by_id(&*state.db, user_claims.uid)
		.await?.to_result()?;
	// let _ = service::session::set(&state.redis, u_id).await?;
	let genTokenReq = GenerateTokenRequest {
		access_key: client.access_key,
		secret_key: client.secret_key,
	};

	let resp = generate(state, genTokenReq).await?;
	info!("New refresh token: {resp:?}");
	Ok(resp)
}


pub async fn generate(state: &AppState,req: GenerateTokenRequest) -> AppResult<TokenResponse> {
	let client = repo::clients::find_by_ak_and_sk(
		&*state.db, req.access_key.as_str(), req.secret_key.as_str())
		.await?.to_result()?;
	// let session_id = service::session::set(&state.redis, client.id).await?;
	let access_token = UserClaims::new(EXPIRE_BEARER_TOKEN_SECS, client.id)
		.encode(&ACCESS_TOKEN_ENCODE_KEY)?;
	let refresh_token = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, client.id)
		.encode(&REFRESH_TOKEN_ENCODE_KEY)?;

	Ok(TokenResponse::new(
		access_token,
		refresh_token,
		EXPIRE_BEARER_TOKEN_SECS.as_secs(),
	))
}

pub async fn validate(state: &AppState, uuid: Uuid) -> AppResult<Option<crate::entities::clients::Model>> {
	Ok(repo::clients::find_by_id(&*state.db, uuid).await?)
}