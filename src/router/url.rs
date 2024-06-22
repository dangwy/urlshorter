use axum::routing::{get, post, delete, patch};
use crate::server::state::AppState;
use crate::handler::urls;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/", post(urls::create))
        .route("/alias/:domain/:alias/info", get(urls::get))
        .route("/alias/:domain/:alias", delete(urls::delete))
        .route("/alias/:domain/:alias", patch(urls::patch))
        .route("/alias/:domain/:alias", get(urls::redirect))
}
