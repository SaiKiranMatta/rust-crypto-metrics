use crate::services::db::Database;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct EarningsQueryParams {
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
    #[schema(example = "asset_earnings")]
    pub sort_by: Option<String>,
    #[schema(example = "asc")]
    pub order: Option<String>,
    #[schema(example = "day")]
    pub interval: Option<String>,
    #[schema(example = false)]
    pub summary: Option<bool>,
}

/// Get pool earnings information
#[utoipa::path(
    get,
    path = "/earnings",
    params(
        ("start_time" = Option<i64>, Query, description = "Start time Unix timestamp"),
        ("end_time" = Option<i64>, Query, description = "End time Unix timestamp"),
        ("pool" = Option<String>, Query, description = "Pool identifier"),
        ("page" = Option<u32>, Query, description = "Page number (minimum: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (1-100)"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by"),
        ("order" = Option<String>, Query, description = "Sort order (asc or desc)"),
        ("interval" = Option<String>, Query, description = "Time interval for aggregation (hour, day, week, month, quarter, year)"),
        ("summary" = Option<bool>, Query, description = "Whether to include summary in response")
    ),
    responses(
        (status = 200, description = "List of pool earnings", body = Vec<EarningsResponse>),
        (status = 400, description = "Bad request - Invalid parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Earnings History"
)]
#[get("/earnings")]
pub async fn get_pool_earnings_api(
    db: Data<Database>,
    query: Query<EarningsQueryParams>,
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

    let include_summary = query.summary.unwrap_or(false);

    match db
        .get_pool_earnings(
            query.start_time,
            query.end_time,
            query.pool.clone(),
            page,
            limit,
            query.sort_by.clone(),
            sort_order,
            query.interval.clone(),
            include_summary,
        )
        .await
    {
        Ok(earnings) => HttpResponse::Ok().json(earnings),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
