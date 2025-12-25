use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "FreeRadical CMS API",
        version = "1.2.0",
        description = "AI-powered CMS with multi-provider support"
    ),
    paths(),
    components(schemas()),
    tags(
        (name = "AI", description = "AI features"),
        (name = "Content", description = "CMS content")
    )
)]
pub struct ApiDoc;
