use crate::services::db::Database;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EarningsQueryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub pool: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub interval: Option<String>,
    pub summary: Option<bool>, // Optional summary parameter
}

#[get("/earnings")]
pub async fn get_pool_earnings_api(
    db: Data<Database>,
    query: Query<EarningsQueryParams>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(10).clamp(1, 100);
    let page = query.page.unwrap_or(1).max(1);
    let sort_order = match query.order.as_deref() {
        Some("asc") => 1,
        Some("desc") => -1,
        _ => 1,
    };
    
   
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
