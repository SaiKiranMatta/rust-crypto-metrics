use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::depth_route::get_pool_depth_price_history,
        crate::routes::earnings_route::get_pool_earnings_api,
        crate::routes::rpmuh_route::get_rune_pool_history
    ),
    components(
        schemas(
            crate::routes::depth_route::DepthHistoryQueryParams, 
            crate::routes::earnings_route::EarningsQueryParams,
            crate::routes::rpmuh_route::RunePoolHistoryQueryParams,
            )
    ),
    tags(
        (name = "Depth and Price History", description = "Endpoint to get depth and price history"),
        (name = "Earnings History", description = "Endpoint to get earnings history"),
        (name = "Rune Pool History", description = "Endpoint to get RunePool total members and units history")
    )
)]
pub struct ApiDoc;