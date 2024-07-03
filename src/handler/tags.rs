use crate::dto::request::*;
use crate::dto::response::*;
use crate::error::AppResult;
use crate::server::state::AppState;
use crate::service;
use axum::extract::{Path, State};
use axum::Json;
use tracing::{info, warn};

/// Create new tag
#[utoipa::path(
	post,
	path = "/tags",
	request_body = CreateTagRequest,
	responses(
		(status = 200, description = "Success create short url", body = [CreateTagResponse]),
		(status = 400, description = "Invalid data input", body = [AppResponseError]),
		(status = 500, description = "Internal server error", body = [AppResponseError])
	)
)]
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateTagRequest>,
) -> AppResult<Json<CreateTagResponse>> {
    info!("Create a new tag with request: {req:?}");
    match service::tags::create(state, req).await {
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

/// Get tag info
#[utoipa::path(
	get,
	path = "/tags",
	responses(
		(status = 200, description = "Success get short url", body = [TagResponse]),
		(status = 400, description = "Invalid data input", body = [AppResponseError]),
		(status = 500, description = "Internal server error", body = [AppResponseError])
	)
)]
pub async fn get(
    State(state): State<AppState>,
    Path((domain, tag)): Path<(String, String)>,
) -> AppResult<Json<TagResponse>> {
    info!("Get tag info with domain and tag: {domain:?}/{tag:?}");
    match service::tags::get(state, &domain, &tag).await {
        Ok(resp) => {
            info!("Get tag info successfully.");
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessful get tag info:: {e:?}");
            Err(e)
        }
    }
}

/// Delete tag info
#[utoipa::path(
	delete,
	path = "/tags",
	responses(
		(status = 200, description = "Success delete short url"),
		(status = 400, description = "Invalid data input", body = [AppResponseError]),
		(status = 500, description = "Internal server error", body = [AppResponseError])
	)
)]
pub async fn delete(
    State(state): State<AppState>,
    Path((domain, tag)): Path<(String, String)>,
) -> AppResult {
    info!("Delete tag info with domain and tag: {domain:?}/{tag:?}");
    match service::tags::delete(state, &domain, &tag).await {
        Ok(_) => {
            info!("Delete tag info successfully.");
            Ok(())
        }
        Err(e) => {
            warn!("Unsuccessful delete tag info:: {e:?}");
            Err(e)
        }
    }
}
