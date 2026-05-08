use crate::handler::__path_health_check;
use crate::handler::word::__path_active_word_handler;
use crate::handler::word::__path_add_word_handler;
use crate::handler::word::__path_word_count_handler;
use utoipa::OpenApi;
use wordbin_types::CreateWord;

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
)]
pub struct ApiDoc;
