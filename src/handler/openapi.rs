use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};

use crate::dto::*;
use crate::error::{AppError, AppResponseError};

#[derive(OpenApi)]
#[openapi(
    info(
        version = "v0.1.0",
        title = "urlshorter API",
    ),
    paths(
    // server api
        crate::handler::server::health_check,
        crate::handler::server::server_state,
    // token api
        crate::handler::token::generate,
        crate::handler::token::refresh,
    // urls api
        crate::handler::urls::create,
        crate::handler::urls::get,
        crate::handler::urls::redirect,
        crate::handler::urls::delete,
    //tags api
        crate::handler::tags::create,
        crate::handler::tags::get,
        crate::handler::tags::delete,
    ),
    components(
        schemas(
            CreateUrlRequest,
            GetUrlQueryParam,
            PatchUrlRequest,
            CreateTagRequest,
            RefreshTokenRequest,
            GenerateTokenRequest,
            UrlResponse,
            RedirectUrlResponse,
            CreateTagResponse,
            ServiceStatusResponse,
            TokenResponse,
            AppResponseError,
            AppError,
            MessageResponse,
        )
    ),
    tags(
        (name = "crate::handler::server", description = "server endpoints."),
        (name = "crate::handler::token", description = "token endpoints."),
        (name = "crate::handler::urls", description = "urls endpoints."),
        (name = "crate::handler::tags", description = "tags endpoints."),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}
