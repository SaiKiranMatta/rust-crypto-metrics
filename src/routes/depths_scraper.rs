use crate::services::{self, db::Database};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FetchDepthParams {
    pub pool: String,        
    pub interval: String,    
    pub start_time: i64,     
}

#[post("/depths_scraper")]
pub async fn fetch_and_store_depth(
    db: Data<Database>,
    params: Json<FetchDepthParams>,
) -> HttpResponse {
    match services::depth_history_fetcher::fetch_and_store_depth_history(&db, &params.pool, &params.interval, params.start_time).await {
        Ok(()) => HttpResponse::Ok().body("Depth history fetched and stored successfully."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
