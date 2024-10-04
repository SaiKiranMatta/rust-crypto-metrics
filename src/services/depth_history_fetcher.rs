use reqwest::Error;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::models::depth_history_model::{PoolDepthPriceHistory, PoolDepthPriceHistoryRequest};

use super::db::Database;


#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    startTime: String,
    endTime: String,
    priceShiftLoss: String,
    luviIncrease: String,
    startAssetDepth: String,
    startRuneDepth: String,
    startLPUnits: String,
    startMemberCount: String,
    startSynthUnits: String,
    endAssetDepth: String,
    endRuneDepth: String,
    endLPUnits: String,
    endMemberCount: String,
    endSynthUnits: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Interval {
    startTime: String,
    endTime: String,
    assetDepth: String,
    runeDepth: String,
    assetPrice: String,
    assetPriceUSD: String,
    liquidityUnits: String,
    membersCount: String,
    synthUnits: String,
    synthSupply: String,
    units: String,
    luvi: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    meta: Meta,
    intervals: Vec<Interval>,
}

async fn store_intervals_in_db(intervals: Vec<Interval>, db: &Database) {
    for interval in intervals {
        let new_depth_history = PoolDepthPriceHistory::try_from(PoolDepthPriceHistoryRequest {
            pool: "BTC.BTC".to_string(), // Assuming this is a valid string
        
            asset_depth: interval.assetDepth.parse::<f64>().expect("Failed to parse assetDepth"),
            asset_price: interval.assetPrice.parse::<f64>().expect("Failed to parse assetPrice"),
            asset_price_usd: interval.assetPriceUSD.parse::<f64>().expect("Failed to parse assetPriceUSD"),
            
            end_time: interval.endTime.parse::<i64>().expect("Failed to parse endTime"),
            start_time: interval.startTime.parse::<i64>().expect("Failed to parse startTime"),
            
            liquidity_units: interval.liquidityUnits.parse::<f64>().expect("Failed to parse liquidityUnits"),
            luvi: interval.luvi.parse::<f64>().expect("Failed to parse luvi"),
            members_count: interval.membersCount.parse::<i64>().expect("Failed to parse membersCount"),
            
            rune_depth: interval.runeDepth.parse::<f64>().expect("Failed to parse runeDepth"),
            synth_supply: interval.synthSupply.parse::<f64>().expect("Failed to parse synthSupply"),
            synth_units: interval.synthUnits.parse::<f64>().expect("Failed to parse synthUnits"),
            units: interval.units.parse::<f64>().expect("Failed to parse units"),
        })
        .unwrap();
        

        match db.create_depth_history(new_depth_history).await {
            Ok(result) => println!("Successfully inserted: {:?}", result.inserted_id),
            Err(e) => eprintln!("Error inserting document: {:?}", e),
        }
    }
}

pub async fn fetch_and_store_depth_history(db: &Database, pool: &String, interval: &String, start_time: i64) -> Result<(), Error> {
    let mut current_time = start_time;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/depths/{}?interval={}&from={}&count=400",
            pool,
            interval, 
            current_time
        );

        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;

        store_intervals_in_db(response.intervals, db).await;
        let end_time: i64 = response.meta.endTime.parse().unwrap();


        let current_utc: DateTime<Utc> = Utc::now();
        let current_timestamp = current_utc.timestamp();

        if end_time >= current_timestamp {
            break;
        }

        current_time = end_time;
    }

    Ok(())
}
