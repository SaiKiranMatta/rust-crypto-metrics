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
pub struct FetchDepthParams {
    pub pool: String,
    pub interval: String,
    pub start_time: i64,
    pub secret: String,
}

#[post("/depths_scraper")]
pub async fn fetch_and_store_depth(
    db: Data<Database>,
    params: Json<FetchDepthParams>,
) -> HttpResponse {
    dotenv().ok();
    
    let expected_secret = env::var("SECRET_KEY").unwrap_or_else(|_| "default_secret".to_string());

    if params.secret != expected_secret {
        return HttpResponse::Unauthorized().body("Wrong secret key.");
    }

    match services::depth_history_fetcher::fetch_and_store_depth_history(&db, &params.pool, &params.interval, params.start_time).await {
        Ok(()) => HttpResponse::Ok().body("Depth history fetched and stored successfully."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
