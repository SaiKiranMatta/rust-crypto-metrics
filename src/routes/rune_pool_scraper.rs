use crate::services::{self, db::Database};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FetchRunePoolParams {
    pub interval: String, // Interval for fetching data
    pub from: i64,       // From timestamp
}

#[post("/rune_pool_scraper")]
pub async fn fetch_and_store_rune_pool(
    db: Data<Database>,
    params: Json<FetchRunePoolParams>,
) -> HttpResponse {
    match services::rpmuh_fetcher::fetch_and_store_rune_pool_history(&db, &params.interval, params.from).await {
        Ok(()) => HttpResponse::Ok().body("Rune pool history fetched and stored successfully."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
