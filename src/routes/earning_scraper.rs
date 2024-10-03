use crate::services::{self, db::Database};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FetchEarningsParams {
    pub interval: String,
    pub start_time: i64,
}

#[post("/earnings_scraper")]
pub async fn fetch_and_store_earnings(
    db: Data<Database>,
    params: Json<FetchEarningsParams>,
) -> HttpResponse {
    match services::earnings_fetcher::fetch_and_store_earnings(&db, &params.interval, params.start_time).await {
        Ok(()) => HttpResponse::Ok().body("Earnings fetched and stored successfully."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
