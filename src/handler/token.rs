use axum::extract::State;
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::utils::claim::UserClaims;
use crate::{dto::*, service};

/// Refresh token.
#[utoipa::path(
	post,
	path = "/token/refresh",
	responses(
		(status = 200, description = "Success get new access token and refresh token", body = [TokenResponse]),
		(status = 400, description = "Invalid data input", body = [AppResponseError]),
		(status = 401, description = "Unauthorized user", body = [AppResponseError]),
		(status = 500, description = "Internal server error", body = [AppResponseError])
	),
)]
pub async fn refresh(State(state): State<AppState>, Json(req): Json<RefreshTokenRequest>
) -> AppResult<Json<TokenResponse>> {
	info!("Refresh token with request: {req:?}.");
	match service::token::refresh(&state, req).await {
		Ok(resp) => {
			info!("Success refresh token user response: {resp:?}.");
			Ok(Json(resp))
		}
		Err(e) => {
			warn!("Unsuccessfully refresh token error: {e:?}.");
			Err(e)
		}
	}
}

/// Generate token.
#[utoipa::path(
	post,
	path = "/token",
	request_body = GenerateTokenRequest,
	responses(
		(status = 200, description = "Success get token information", body = [TokenResponse]),
		(status = 400, description = "Invalid token", body = [AppResponseError]),
		(status = 401, description = "Unauthorized user", body = [AppResponseError]),
		(status = 500, description = "Internal server error", body = [AppResponseError])
	),
)]
pub async fn generate(State(state): State<AppState>, Json(req): Json<GenerateTokenRequest>)
-> AppResult<Json<TokenResponse>>{
	info!("Generate token with request: {req:?}.");
	match service::token::generate(&state, req).await {
		Ok(resp) => {
			info!("Success generate token response: {resp:?}.");
			Ok(Json(resp))
		}
		Err(e) => {
			warn!("Unsuccessfully generate token error: {e:?}.");
			Err(e)
		}
	}
}