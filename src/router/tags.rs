use axum::routing::{get, post, delete};
use crate::server::state::AppState;
use crate::handler::tags;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
	router
		.route("/tags", post(tags::create))
		.route("/tags/:domain/:tag", get(tags::get))
		.route("/tags/:domain/:tag", delete(tags::delete))

}