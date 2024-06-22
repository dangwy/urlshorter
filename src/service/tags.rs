use chrono::{NaiveDateTime, Utc};
use sea_orm::TransactionTrait;
use crate::dto::request::CreateTagRequest;
use crate::dto::response::{CreateTagResponse, TagResponse};
use crate::error::{AppResult, invalid_input_error};
use crate::server::state::AppState;
use crate::{repo, service};

pub async fn create(state: AppState, req: CreateTagRequest) -> AppResult<CreateTagResponse> {
	let tx = state.db.begin().await?;
	let model = repo::tags::find_by_tag(&state.db, &req.domain, &req.tag).await?;
	let tag = if let Some(tags) = model {
		CreateTagResponse {
			id: tags.id
		}
	}else{
		let tag_id = repo::tags::save(&tx, &req.domain, &req.tag).await?;
		CreateTagResponse {
			id: tag_id,
		}
	};
	tx.commit().await?;
	Ok(tag)
}

pub async fn get(state: AppState, domain: &str, tag: &str) -> AppResult<TagResponse> {
	let model = repo::tags::find_by_tag(&state.db, &domain, &tag).await?;
	if let Some(res) = model {
		let url = TagResponse {
			id: res.id,
			tag: res.tag,
			domain: res.domain,
			created_at: res.created_at,
		};
		Ok(url)
	}else {
		Err(invalid_input_error("alias","Url not found."))
	}
}

pub async fn delete(state: AppState, domain: &str, tag: &str) -> AppResult<> {
	let tx = state.db.begin().await?;
	let _ = repo::tags::delete_by_tag(&tx, &domain, &tag).await?;
	tx.commit().await?;
	Ok(())
}