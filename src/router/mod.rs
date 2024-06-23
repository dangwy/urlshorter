use crate::server::state::AppState;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod tags;
pub mod url;

pub fn create_router_app(state: AppState) -> Router {
    let router = Router::new();
    // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
    let router = url::add_routers(router);
    let router = tags::add_routers(router);
    router.with_state(state)
}
