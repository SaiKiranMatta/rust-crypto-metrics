use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::depth_route::get_pool_depth_price_history
    ),
    components(
        schemas(crate::routes::depth_route::DepthHistoryQueryParams)
    ),
    tags(
        (name = "Depth and Price History", description = "Endpoint to get depth and price history")
    )
)]
pub struct ApiDoc;