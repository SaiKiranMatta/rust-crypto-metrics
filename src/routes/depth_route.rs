use crate::services::db::Database;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HistoryQueryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub pool: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
}

#[get("/depths")]
pub async fn get_pool_depth_price_history(
    db: Data<Database>,
    query: Query<HistoryQueryParams>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(10).clamp(1, 100);
    let page = query.page.unwrap_or(1).max(1);
    let sort_order = match query.order.as_deref() {
        Some("asc") => 1,
        Some("desc") => -1,
        _ => 1,
    };

    match db
        .get_pool_depth_price_history(
            query.start_time,
            query.end_time,
            query.pool.clone(),
            page,
            limit,
            query.sort_by.clone(),
            sort_order,
        )
        .await
    {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
