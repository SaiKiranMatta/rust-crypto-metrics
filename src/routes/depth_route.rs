use crate::services::db::Database;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HistoryQueryParams {
    pub interval: Option<String>, 
    pub count: Option<u32>,       
    pub from: Option<i64>,        
    pub to: Option<i64>,          
}

#[get("/depths")]
pub async fn get_pool_depth_price_history(
    db: Data<Database>,
    query: Query<HistoryQueryParams>,
) -> HttpResponse {
    if let Some(interval) = &query.interval {
        if !["5min", "hour", "day", "week", "month", "quarter", "year"].contains(&interval.as_str()) {
            return HttpResponse::BadRequest().body(format!("Invalid interval: {}", interval));
        }
    }

    if let Some(count) = query.count {
        if count < 1 || count > 400 {
            return HttpResponse::BadRequest().body("Count must be between 1 and 400.");
        }
    }

    match db
        .get_pool_depth_price_history(
            query.interval.clone(),
            query.count,
            query.from,
            query.to,
        )
        .await
    {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
