use crate::services::{self, db::Database};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FetchSwapsParams {
    pub interval: String,
    pub start_time: i64,
    pub pool: String,  // Added to specify the pool for fetching swap history
}

#[post("/swaps_scraper")]
pub async fn fetch_and_store_swaps(
    db: Data<Database>,
    params: Json<FetchSwapsParams>,
) -> HttpResponse {
    match services::swaps_history_fetcher::fetch_and_store_swaps_history(&db, &params.pool, &params.interval, params.start_time).await {
        Ok(()) => HttpResponse::Ok().body("Swaps fetched and stored successfully."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
