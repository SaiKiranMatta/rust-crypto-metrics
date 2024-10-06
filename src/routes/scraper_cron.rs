use crate::services::{self, db::Database};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct CronParams {
    pub secret: String,
    pub pool: String,
}

#[post("/scrape_all")]
pub async fn run_all_jobs(
    db: Data<Database>,
    params: Json<CronParams>,
) -> HttpResponse {
    dotenv().ok();
    
    let expected_secret = env::var("SECRET_KEY").unwrap_or_else(|_| "default_secret".to_string());

    let interval = "hour".to_string();

    if params.secret != expected_secret {
        return HttpResponse::Unauthorized().body("Wrong secret key.");
    }

    let current_time = Utc::now().timestamp();
    let one_hour_ago = current_time - 3600;

    let swap_result = services::swaps_history_fetcher::fetch_and_store_swaps_history(
        &db,
        &params.pool,
        &interval,
        one_hour_ago,
    ).await;

    let rune_pool_result = services::rpmuh_fetcher::fetch_and_store_rune_pool_history(
        &db,
        &interval,
        one_hour_ago,
    ).await;

    let earnings_result = services::earnings_fetcher::fetch_and_store_earnings(
        &db,
        &interval,
        one_hour_ago,
    ).await;

    let depth_result = services::depth_history_fetcher::fetch_and_store_depth_history(
        &db,
        &params.pool,
        &interval,
        one_hour_ago,
    ).await;

    let results = vec![swap_result, rune_pool_result, earnings_result, depth_result];

    for result in results {
        if result.is_err() {
            return HttpResponse::InternalServerError().body("One or more jobs failed.");
        }
    }

    HttpResponse::Ok().body("All jobs executed successfully.")
}
