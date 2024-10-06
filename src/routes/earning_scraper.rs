use crate::services::{self, db::Database};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct FetchEarningsParams {
    pub interval: String,
    pub start_time: i64,
    pub secret: String,
}

#[post("/earnings_scraper")]
pub async fn fetch_and_store_earnings(
    db: Data<Database>,
    params: Json<FetchEarningsParams>,
) -> HttpResponse {
    dotenv().ok();

    let expected_secret = env::var("SECRET_KEY").unwrap_or_else(|_| "default_secret".to_string());

    if params.secret != expected_secret {
        return HttpResponse::Unauthorized().body("Wrong secret key.");
    }

    match services::earnings_fetcher::fetch_and_store_earnings(&db, &params.interval, params.start_time).await {
        Ok(()) => HttpResponse::Ok().body("Earnings fetched and stored successfully."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
