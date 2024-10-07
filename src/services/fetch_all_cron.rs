use crate::services::{self, db::Database};
use actix_web::web::Data;
use chrono:: Utc;
use tokio::time::{interval, Duration}; // For handling the interval.
use dotenv::dotenv;

pub async fn run_cron_job(db: Data<Database>, pool: String) {
    dotenv().ok();
    
    let mut interval = interval(Duration::from_secs(3600)); // 1 hour interval
    
    loop {
        interval.tick().await; // Wait for the next tick
        let start_time = Utc::now();
        let one_hour_ago = start_time.timestamp() - 3600; // Get the timestamp for one hour ago
        let interval_str = "hour".to_string();
        
        println!("Running scheduled data fetch at {:?}", start_time);
        
        let swap_result = services::swaps_history_fetcher::fetch_and_store_swaps_history(
            &db,
            &pool,
            &interval_str,
            one_hour_ago,
        )
        .await;
    
        let rune_pool_result = services::rpmuh_fetcher::fetch_and_store_rune_pool_history(
            &db,
            &interval_str,
            one_hour_ago,
        )
        .await;

        let earnings_result = services::earnings_fetcher::fetch_and_store_earnings(
            &db,
            &interval_str,
            one_hour_ago,
        )
        .await;

        let depth_result = services::depth_history_fetcher::fetch_and_store_depth_history(
            &db,
            &pool,
            &interval_str,
            one_hour_ago,
        )
        .await;

        let results = vec![swap_result, rune_pool_result, earnings_result, depth_result];

        let mut has_error = false;
        for result in results {
            if let Err(e) = result {
                eprintln!("Error occurred during data fetch: {:?}", e);
                has_error = true;
            }
        }

        if has_error {
            eprintln!("One or more errors occurred during the job. Aborting this cycle.");
        } else {
            println!("All data fetch tasks completed successfully.");
        }

        let end_time = Utc::now();
        println!("Data fetch completed at {:?}, duration: {:?}", end_time, end_time - start_time);
        // println!("Cron job running");
    }
}
