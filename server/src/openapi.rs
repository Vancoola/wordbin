use crate::handler::__path_health_check;
use crate::handler::word::__path_active_word_handler;
use crate::handler::word::__path_add_word_handler;
use crate::handler::word::__path_word_count_handler;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use wordbin_types::word::CreateWord;

#[derive(OpenApi)]
#[openapi(
    paths (
        add_word_handler,
        word_count_handler,
        active_word_handler,
        health_check
    ),
    components (
        schemas (
            CreateWord
        )
    ),
    tags (
        (name="word", description=""),
        (name="auth", description=""),
        (name="status", description="")
    ),
    info(
        title="Wordbin API",
        version="0.1.0",
        description="API Documentation",
        license(
            name="Apache-2.0",
            url="https://www.apache.org/licenses/",
        ),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
