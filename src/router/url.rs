use axum::routing::{get, post, delete};

use crate::handler::urls;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/", post(urls::create))
        .route("/alias/:domain/:alias", get(urls::get))
        .route("/alias/:domain/:alias", delete(urls::delete))
        // .route("/", get(alias::redirect))
}
