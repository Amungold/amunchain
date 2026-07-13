use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "AmunChain RPC API",
        version = "1.0.0",
        description = "Official RPC API for AmunChain — Sovereign Constitutional Protocol"
    ),
    tags(
        (name = "Network", description = "Network status endpoints"),
        (name = "Blocks", description = "Block query endpoints"),
        (name = "Explorer", description = "Explorer-specific endpoints"),
    )
)]
pub struct ApiDoc;
