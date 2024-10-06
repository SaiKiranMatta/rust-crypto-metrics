#![recursion_limit = "512"]

mod services;
mod models;
mod routes;

use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use routes::depth_route::get_pool_depth_price_history;
use routes::depths_scraper::fetch_and_store_depth;
use routes::earning_scraper:: fetch_and_store_earnings;
use routes::earnings_route::get_pool_earnings_api;
use routes::rpmuh_route::get_rune_pool_history;
use routes::rune_pool_scraper::fetch_and_store_rune_pool;
use routes::scraper_cron::run_all_jobs;
use routes::swaps_route::get_pool_swap_history;
use routes::swaps_scraper::fetch_and_store_swaps;
use services::db::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello S!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(get_pool_depth_price_history)
            .service(get_pool_swap_history)
            .service(get_rune_pool_history)
            .service(get_pool_earnings_api)
            .service(fetch_and_store_earnings)
            .service(fetch_and_store_swaps)
            .service(fetch_and_store_depth)
            .service(fetch_and_store_rune_pool)
            .service(run_all_jobs)
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
