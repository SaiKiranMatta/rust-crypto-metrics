use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::depth_route::get_pool_depth_price_history,
        crate::routes::earnings_route::get_pool_earnings_api
    ),
    components(
        schemas(
            crate::routes::depth_route::DepthHistoryQueryParams, 
            crate::routes::earnings_route::EarningsQueryParams
            )
    ),
    tags(
        (name = "Depth and Price History", description = "Endpoint to get depth and price history"),
        (name = "Earnings History", description = "Endpoint to get earnings history")
    )
)]
pub struct ApiDoc;