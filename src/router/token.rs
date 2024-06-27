use axum::routing::post;

use crate::handler::token;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
	router
		.route("/token/refresh", post(token::refresh))
		.route("/token", post(token::generate))
}
