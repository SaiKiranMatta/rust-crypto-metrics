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
pub struct FetchRunePoolParams {
    pub interval: String,
    pub from: i64,
    pub secret: String,
}

#[post("/rune_pool_scraper")]
pub async fn fetch_and_store_rune_pool(
    db: Data<Database>,
    params: Json<FetchRunePoolParams>,
) -> HttpResponse {
    dotenv().ok();

    let expected_secret = env::var("SECRET_KEY").unwrap_or_else(|_| "default_secret".to_string());

    if params.secret != expected_secret {
        return HttpResponse::Unauthorized().body("Wrong secret key.");
    }

    match services::rpmuh_fetcher::fetch_and_store_rune_pool_history(&db, &params.interval, params.from).await {
        Ok(()) => HttpResponse::Ok().body("Rune pool history fetched and stored successfully."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
