use crate::services::db::Database;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;


#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SwapHistoryQueryParams {
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
    #[schema(example = "synth_mint_volume")]
    pub sort_by: Option<String>,
    #[schema(example = "asc")]
    pub order: Option<String>,
    #[schema(example = "day")]
    pub interval: Option<String>,
}

#[derive(utoipa::ToSchema)]
#[allow(dead_code)]
pub struct PoolSwapHistoryResponse {
    /// The pool identifier (e.g., "BTC.BTC")
    #[schema(example = "BTC.BTC")]
    pub pool: String,

    /// Start time of the swap history period (UNIX timestamp)
    #[schema(example = 1728194400)]
    pub start_time: i64,

    /// End time of the swap history period (UNIX timestamp)
    #[schema(example = 1728198000)]
    pub end_time: i64,

    /// Number of swaps to asset
    #[schema(example = 2)]
    pub to_asset_count: i64,

    /// Number of swaps to Rune
    #[schema(example = 24)]
    pub to_rune_count: i64,

    /// Number of trades to assets
    #[schema(example = 16)]
    pub to_trade_count: i64,

    /// Number of trades from assets
    #[schema(example = 15)]
    pub from_trade_count: i64,

    /// Number of synthetic mints
    #[schema(example = 0)]
    pub synth_mint_count: i64,

    /// Number of synthetic redemptions
    #[schema(example = 0)]
    pub synth_redeem_count: i64,

    /// Total number of swaps and trades
    #[schema(example = 57)]
    pub total_count: i64,

    /// Total volume swapped to asset
    #[schema(example = 837606947909.0)]
    pub to_asset_volume: f64,

    /// Total volume swapped to Rune
    #[schema(example = 247724011940.0)]
    pub to_rune_volume: f64,

    /// Total trade volume to assets
    #[schema(example = 1756851052903.0)]
    pub to_trade_volume: f64,

    /// Total trade volume from assets
    #[schema(example = 3063215459056.0)]
    pub from_trade_volume: f64,

    /// Total synthetic mint volume
    #[schema(example = 0.0)]
    pub synth_mint_volume: f64,

    /// Total synthetic redemption volume
    #[schema(example = 0.0)]
    pub synth_redeem_volume: f64,

    /// Total trade volume (including swaps and synthetic transactions)
    #[schema(example = 5905397471808.0)]
    pub total_volume: f64,

    /// USD value of volume swapped to asset
    #[schema(example = 4025299.0)]
    pub to_asset_volume_usd: f64,

    /// USD value of volume swapped to Rune
    #[schema(example = 1191042.0)]
    pub to_rune_volume_usd: f64,

    /// USD value of trades to assets
    #[schema(example = 8455774.0)]
    pub to_trade_volume_usd: f64,

    /// USD value of trades from assets
    #[schema(example = 14731284.0)]
    pub from_trade_volume_usd: f64,

    /// USD value of synthetic mint volume
    #[schema(example = 0.0)]
    pub synth_mint_volume_usd: f64,

    /// USD value of synthetic redemption volume
    #[schema(example = 0.0)]
    pub synth_redeem_volume_usd: f64,

    /// Total USD value of trade volume
    #[schema(example = 28403399.0)]
    pub total_volume_usd: f64,

    /// Fees for swaps to asset
    #[schema(example = 1255378111.0)]
    pub to_asset_fees: f64,

    /// Fees for swaps to Rune
    #[schema(example = 371586008.0)]
    pub to_rune_fees: f64,

    /// Fees for trades to assets
    #[schema(example = 878220304.0)]
    pub to_trade_fees: f64,

    /// Fees for trades from assets
    #[schema(example = 1712653058.0)]
    pub from_trade_fees: f64,

    /// Fees for synthetic mint transactions
    #[schema(example = 0.0)]
    pub synth_mint_fees: f64,

    /// Fees for synthetic redemption transactions
    #[schema(example = 0.0)]
    pub synth_redeem_fees: f64,

    /// Total fees collected
    #[schema(example = 4217837481.0)]
    pub total_fees: f64,

    /// Average slip for swaps to asset
    #[schema(example = 15.0)]
    pub to_asset_average_slip: f64,

    /// Average slip for swaps to Rune
    #[schema(example = 15.0)]
    pub to_rune_average_slip: f64,

    /// Average slip for trades to assets
    #[schema(example = 5.0)]
    pub to_trade_average_slip: f64,

    /// Average slip for trades from assets
    #[schema(example = 5.13333333333333)]
    pub from_trade_average_slip: f64,

    /// Average slip for synthetic mint transactions
    #[schema(example = 0.0)]
    pub synth_mint_average_slip: f64,

    /// Average slip for synthetic redemption transactions
    #[schema(example = 0.0)]
    pub synth_redeem_average_slip: f64,

    /// Total average slip
    #[schema(example = 9.59649122807018)]
    pub average_slip: f64,

    /// Price of Rune in USD
    #[schema(example = 4.80524525940677)]
    pub rune_price_usd: f64,
}

/// Get pool swap history
#[utoipa::path(
    get,
    path = "/swaps",
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
        (status = 200, description = "List of pool swap history", body = Vec<PoolSwapHistoryResponse>),
        (status = 400, description = "Bad request - Invalid parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Swaps History"
)]
#[get("/swaps")]
pub async fn get_pool_swap_history(
    db: Data<Database>,
    query: Query<SwapHistoryQueryParams>,
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
        .get_pool_swap_history(
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
