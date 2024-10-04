use crate::services::db::Database;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RunePoolHistoryQueryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
}

#[get("/rune_pool_history")]
pub async fn get_rune_pool_history(
    db: Data<Database>,
    query: Query<RunePoolHistoryQueryParams>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(10).clamp(1, 100);
    let page = query.page.unwrap_or(1).max(1);
    let sort_order = match query.order.as_deref() {
        Some("asc") => 1,
        Some("desc") => -1,
        _ => 1,
    };

    match db
        .get_rune_pool_history(
            query.start_time,
            query.end_time,
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
