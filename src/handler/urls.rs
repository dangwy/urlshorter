use axum::extract::{Query, State, Path};
use axum::Json;
// use axum::routing::{get, path};
use garde::Validate;
use tracing::{info, warn};

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::utils::claim::UserClaims;
use crate::dto::request::*;
use crate::dto::response::*;
use crate::service;

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
pub async fn create(State(state): State<AppState>, Json(req): Json<CreateUrlRequest>,)
    -> AppResult<Json<UrlResponse>> {
    info!("Create a new short link with request: {req:?}");
    req.validate(&())?;
    match service::url::create(state, req).await {
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
    params(GetUrlQueryParam),
    responses(
        (status = 200, description = "Success get url", body = [UrlResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
// Query(param): Query<GetUrlQueryParam>,
pub async fn get(State(state): State<AppState>, Path((domain,alias)): Path<(String,String)> )
    -> AppResult<Json<UrlResponse>> {
    info!("Get url info with domain and alias: {domain:?}/{alias:?}.");
    match service::url::get(state, &domain, &alias).await {
        Ok(resp) => {
            info!("Get url info successfully.");
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessful get url info:: {e:?}");
            Err(e)
        }
    }
}

/*
/// Delete short url.
#[utoipa::path(
    delete,
    request_body = ActiveRequest,
    path = "/alias",
    responses(
    (status = 200, description = "Success active user", body = [MessageResponse]),
    (status = 400, description = "Invalid data input", body = [AppResponseError]),
    (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn delete(State(state): State<AppState>, Json(req): Json<ActiveRequest>,) -> AppResult<Json<UrlResponse>> {
    info!("Active user with token: {req:?}.");
    match alias::get(&state, req).await {
        Ok(_) => {
            info!("User successfully activated.");
            Ok(Json(MessageResponse::new("User successfully activated.")))
        }
        Err(e) => {
            info!("The user activation operation was not successful: {e:?}");
            Err(e)
        }
    }
}

/// Redirect to short url.
#[utoipa::path(
    get,
    request_body = ActiveRequest,
    path = "/alias2",
    responses(
    (status = 200, description = "Success active user", body = [MessageResponse]),
    (status = 400, description = "Invalid data input", body = [AppResponseError]),
    (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn redirect(State(state): State<AppState>, Json(req): Json<ActiveRequest>,) -> AppResult<Json<UrlResponse>> {
    info!("Active user with token: {req:?}.");
    match alias::get(&state, req).await {
        Ok(_) => {
            info!("User successfully activated.");
            Ok(Json(MessageResponse::new("User successfully activated.")))
        }
        Err(e) => {
            info!("The user activation operation was not successful: {e:?}");
            Err(e)
        }
    }
}
*/