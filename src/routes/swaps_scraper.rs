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
pub struct FetchSwapsParams {
    pub interval: String,
    pub start_time: i64,
    pub pool: String,
    pub secret: String,
}

#[post("/swaps_scraper")]
pub async fn fetch_and_store_swaps(
    db: Data<Database>,
    params: Json<FetchSwapsParams>,
) -> HttpResponse {
    dotenv().ok();

    let expected_secret = env::var("SECRET_KEY").unwrap_or_else(|_| "default_secret".to_string());

    if params.secret != expected_secret {
        return HttpResponse::Unauthorized().body("Wrong secret key.");
    }

    match services::swaps_history_fetcher::fetch_and_store_swaps_history(&db, &params.pool, &params.interval, params.start_time).await {
        Ok(()) => HttpResponse::Ok().body("Swaps fetched and stored successfully."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
