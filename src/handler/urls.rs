use axum::extract::{Path, State};
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::dto::request::*;
use crate::dto::response::*;
use crate::error::AppResult;
use crate::error::AppError;
use crate::server::state::AppState;
use crate::service;
use crate::utils::claim::UserClaims;

/// Create short url
#[utoipa::path(
    post,
    path = "/",
    request_body = CreateUrlRequest,
    responses(
        (status = 200, description = "Success create short url", body = [UrlResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn create(
    State(state): State<AppState>,
    client: UserClaims,
    Json(req): Json<CreateUrlRequest>,
) -> AppResult<Json<UrlResponse>> {
    info!("Create a new short link with request: {req:?}");
    req.validate(&())?;
    match service::url::create(state, client.uid, req).await {
        Ok(resp) => {
            info!("Successfully create link: ");
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully create link: {e:?}");
            Err(e)
        }
    }
}

/// Get short url info.
#[utoipa::path(
    get,
    path = "/alias",
    responses(
        (status = 200, description = "Success get url", body = [UrlResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn get(
    State(state): State<AppState>,
    Path((domain, alias)): Path<(String, String)>,
    client: UserClaims,
) -> AppResult<Json<UrlResponse>> {
    info!("Get url info with domain and alias: {domain:?}/{alias:?}");
    let clients = service::token::validate(&state, client.uid).await?;
    if let Some(_) = clients {
        info!("client valid: {client:?}");
        match service::url::get(state, client.uid, &domain, &alias).await {
            Ok(resp) => {
                info!("Get url info successfully.");
                Ok(Json(resp))
            }
            Err(e) => {
                warn!("Unsuccessful get url info:: {e:?}");
                Err(e)
            }
        }
    }else {
        Err(AppError::InvalidInputError(garde::Report::new()))
    }
}

/// Delete short url.
#[utoipa::path(
    delete,
    path = "/alias",
    responses(
        (status = 200, description = "Success active user"),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    Path((domain, alias)): Path<(String, String)>,
    client: UserClaims
) -> AppResult {
    info!("Delete short url alias: {domain:?}/{alias:?}");
    match service::url::delete(state, client.uid, &domain, &alias).await {
        Ok(_) => {
            info!("Delete short url successfully");
            Ok(())
        }
        Err(e) => {
            info!("Unsuccessful delete short url info: {e:?}");
            Err(e)
        }
    }
}

/// Patch short url.
#[utoipa::path(
    patch,
    path = "/alias",
    request_body = PatchUrlRequest,
    responses(
        (status = 200, description = "Success active user", body = [UrlResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn patch(
    State(state): State<AppState>,
    Path((domain, alias)): Path<(String, String)>,
    Json(req): Json<PatchUrlRequest>,
    client: UserClaims,
) -> AppResult<Json<UrlResponse>> {
    info!("patch short url: {domain:?}/{alias:?}");
    match service::url::patch(state, client.uid, &domain, &alias, req).await {
        Ok(resp) => {
            info!("Patch url info successfully.");
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessful patch short url info:: {e:?}");
            Err(e)
        }
    }
}

/// Redirect to short url.
#[utoipa::path(
    get,
    path = "/alias2",
    responses(
        (status = 200, description = "Success redirect to original url", body = [RedirectUrlResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn redirect(
    State(state): State<AppState>,
    Path((domain, alias)): Path<(String, String)>,
    client: UserClaims,
) -> AppResult<Json<RedirectUrlResponse>> {
    info!("Redirect to original url with token: {domain:?}/{alias:?}.");
    match service::url::redirect(state, client.uid, &domain, &alias).await {
        Ok(resp) => {
            info!("User successfully redirected.");
            Ok(Json(resp))
        }
        Err(e) => {
            info!("The user activation operation was not successful: {e:?}");
            Err(e)
        }
    }
}
