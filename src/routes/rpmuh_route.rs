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
    pub interval: Option<String>,
}

#[get("/rune_pool_history")]
pub async fn get_rune_pool_history(
    db: Data<Database>,
    query: Query<RunePoolHistoryQueryParams>,
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
        .get_rune_pool_history(
            query.start_time,
            query.end_time,
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
