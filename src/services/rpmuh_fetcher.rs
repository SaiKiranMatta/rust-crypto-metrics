use reqwest::Error;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use crate::models::rpmuh_model::{RunePoolHistory, RunePoolHistoryRequest};
use super::db::Database;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct RunePoolMeta {
    pub startTime: String,
    pub endTime: String,
    pub startUnits: String,
    pub startCount: String,
    pub endUnits: String,
    pub endCount: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RunePoolInterval {
    pub startTime: String,
    pub endTime: String,
    pub count: String,
    pub units: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RunePoolHistoryApiResponse {
    pub meta: RunePoolMeta,
    pub intervals: Vec<RunePoolInterval>,
}

pub async fn fetch_and_store_rune_pool_history(db: &Database, interval: &str, mut from: i64) -> Result<(), Error> {
    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool/?interval={}&from={}&count=400",
            interval, 
            from
        );

        let response: RunePoolHistoryApiResponse = reqwest::get(&url).await?.json().await?;

        for interval_data in response.intervals {
            let rune_pool_interval_request = RunePoolHistoryRequest {
                start_time: interval_data.startTime,
                end_time: interval_data.endTime,
                units: interval_data.units,
                count: interval_data.count,
            };

            let rune_pool_interval = RunePoolHistory::try_from(rune_pool_interval_request).unwrap();

            match db.create_rpmuh(rune_pool_interval).await {
                Ok(result) => println!("Successfully inserted: {:?}", result.inserted_id),
                Err(e) => eprintln!("Error inserting document: {:?}", e),
            }
        }

        let end_time: i64 = response.meta.endTime.parse().unwrap();

        let current_utc: DateTime<Utc> = Utc::now();
        let current_timestamp = current_utc.timestamp();

        if end_time >= current_timestamp {
            break;
        }

        from = end_time;
    }

    Ok(())
}
