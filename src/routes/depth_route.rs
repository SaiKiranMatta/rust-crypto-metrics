use crate::services::db::Database;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;
use utoipa::openapi::schema;


#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct DepthHistoryQueryParams {
    #[schema(example = 1653373410)]
    pub start_time: Option<i64>,
    #[schema(example = 1666592610)]
    pub end_time: Option<i64>,
    #[schema(example = "BTC.BTC")]
    pub pool: Option<String>,
    #[schema(example = 1, minimum = 1)]
    pub page: Option<u32>,
    #[schema(example = 10, minimum = 1, maximum = 100)]
    pub limit: Option<u32>,
    #[schema(example = "asset_price")]
    pub sort_by: Option<String>,
    #[schema(example = "asc")]
    pub order: Option<String>,
    #[schema(example = "day")]
    pub interval: Option<String>,
}

#[derive(utoipa::ToSchema)]
#[allow(dead_code)]
pub struct PoolDepthPriceHistoryResponse {
    /// The identifier of the pool (e.g., "BTC.BTC")
    #[schema(example= "BTC.BTC")]
    pub pool: String, 

    /// Depth of the asset in the pool
    #[schema(example = 123456.78)]
    pub asset_depth: f64,

    /// Price of the asset in the pool's base currency
    #[schema(example = 42.50)]
    pub asset_price: f64,

    /// Price of the asset in USD
    #[schema(example = 60000.99)]
    pub asset_price_usd: f64,

    /// Start time of the price history period (UNIX timestamp)
    #[schema(example = 1653373410)]
    pub start_time: Option<i64>,

    /// End time of the price history period (UNIX timestamp)
    #[schema(example = 1666592610)]
    pub end_time: Option<i64>,

    /// The total liquidity units in the pool
    #[schema(example = 100000.0)]
    pub liquidity_units: f64,

    /// Liquidity Utilization Value Indicator (LUVI)
    #[schema(example = 1.75)]
    pub luvi: f64,

    /// Number of members (liquidity providers) in the pool
    #[schema(example = 250)]
    pub members_count: i64,

    /// The amount of Rune in the pool
    #[schema(example = 987654.32)]
    pub rune_depth: f64,

    /// The total supply of synth assets in the pool
    #[schema(example = 5678.90)]
    pub synth_supply: f64,

    /// The number of synthetic asset units in the pool
    #[schema(example = 1234.56)]
    pub synth_units: f64,

    /// Total number of units in the pool
    #[schema(example = 999999.99)]
    pub units: f64,
}


/// Get pool depth price history
#[utoipa::path(
    get,
    path = "/depths",
    params(
        ("start_time" = Option<i64>, Query, description = "Start time Unix timestamp"),
        ("end_time" = Option<i64>, Query, description = "End time Unix timestamp"),
        ("pool" = Option<String>, Query, description = "Pool identifier"),
        ("page" = Option<u32>, Query, description = "Page number (minimum: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (1-100)"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by"),
        ("order" = Option<String>, Query, description = "Sort order (asc or desc)"),
        ("interval" = Option<String>, Query, description = "Time interval for aggregation (hour, day, week, month, quarter, year)")
    ),
    responses(
        (status = 200, description = "List of pool depth price history", body = Vec<PoolDepthPriceHistoryResponse>),
        (status = 400, description = "Bad request - Invalid parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Depth and Price History"
)]
#[get("/depths")]
pub async fn get_pool_depth_price_history(
    db: Data<Database>,
    query: Query<DepthHistoryQueryParams>,
) -> HttpResponse {
    if let (Some(start), Some(end)) = (query.start_time, query.end_time) {
        if start >= end {
            return HttpResponse::BadRequest().body("start_time must be less than end_time.");
        }
    }

    let limit = query.limit.unwrap_or(10).clamp(1, 100);
    if limit < 1 {
        return HttpResponse::BadRequest().body("limit must be a positive integer.");
    }

    let page = query.page.unwrap_or(1).max(1);
    if page < 1 {
        return HttpResponse::BadRequest().body("page must be a positive integer.");
    }

    let sort_order = match query.order.as_deref() {
        Some("asc") => 1,
        Some("desc") => -1,
        _ => 1,
    };

    let valid_ordering = vec!["asc", "desc"];
    if let Some(ref order) = query.order {
        if !valid_ordering.contains(&order.as_str()) {
            return HttpResponse::BadRequest().body(format!("order must be one of: {:?}", valid_ordering));
        }
    }

    let valid_interval = vec!["hour", "day", "week", "month", "quarter", "year"];
    if let Some(ref interval) = query.interval {
        if !valid_interval.contains(&interval.as_str()) {
            return HttpResponse::BadRequest().body(format!("interval must be one of: {:?}", valid_interval));
        }
    }

    match db
        .get_pool_depth_price_history(
            query.start_time,
            query.end_time,
            query.pool.clone(),
            page,
            limit,
            query.sort_by.clone(),
            sort_order,
            query.interval.clone()
        )
        .await
    {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}