use crate::handler::urls;
use crate::server::state::AppState;
use axum::routing::{delete, get,  post};

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/", post(urls::create))
        .route("/alias/:domain/:alias/info", get(urls::get))
        .route("/alias/:domain/:alias", delete(urls::delete))
        // .route("/alias/:domain/:alias", patch(urls::patch))
        .route("/alias/:domain/:alias", get(urls::redirect))
}
