use crate::handler::CreateWord;
use crate::handler::word::__path_add_word;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths (
        add_word
    ),
    components (
        schemas (
            CreateWord
        )
    ),
    tags (
        (name="word", description="")
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
