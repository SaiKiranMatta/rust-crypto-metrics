use crate::services::db::Database;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SwapHistoryQueryParams {
    pub pool: Option<String>,
    pub count: Option<u32>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[get("/swaps")]
pub async fn get_pool_swap_history(
    db: Data<Database>,
    query: Query<SwapHistoryQueryParams>,
) -> HttpResponse {
    if let Some(count) = query.count {
        if count < 1 || count > 400 {
            return HttpResponse::BadRequest().body("Count must be between 1 and 400.");
        }
    }

    match db
        .get_pool_swap_history(
            query.pool.clone(),
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
