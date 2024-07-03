use crate::constant::*;
use crate::dto::response::TokenResponse;
use crate::dto::{GenerateTokenRequest, RefreshTokenRequest};
use crate::error::{AppResult, ToAppResult};
use crate::server::state::AppState;
use crate::repo;
use crate::utils::claim::UserClaims;
use tracing::info;

pub async fn refresh(state: &AppState, req: RefreshTokenRequest) -> AppResult<TokenResponse> {
	let user_claims = UserClaims::decode(&req.token, &REFRESH_TOKEN_DECODE_KEY)?.claims;
	info!("Refresh token: {user_claims:?}");
	let client = repo::clients::find_by_id(&*state.db, user_claims.uid)
		.await?.to_result()?;
	let token_req = GenerateTokenRequest {
		access_key: client.access_key,
		secret_key: client.secret_key,
	};

	let resp = generate(state, token_req).await?;
	info!("New refresh token: {resp:?}");
	Ok(resp)
}


pub async fn generate(state: &AppState,req: GenerateTokenRequest) -> AppResult<TokenResponse> {
	let client = repo::clients::find_by_ak_and_sk(
		&*state.db, req.access_key.as_str(), req.secret_key.as_str())
		.await?.to_result()?;
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

pub async fn validate(state: &AppState, uuid: i64) -> AppResult<Option<crate::entities::clients::Model>> {
	Ok(repo::clients::find_by_id(&*state.db, uuid).await?)
}