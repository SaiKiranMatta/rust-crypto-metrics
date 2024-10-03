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
            pool: "BTC.BTC".to_string(),
            asset_depth: interval.assetDepth,
            asset_price: interval.assetPrice,
            asset_price_usd: interval.assetPriceUSD,
            end_time: interval.endTime,
            liquidity_units: interval.liquidityUnits,
            luvi: interval.luvi,
            members_count: interval.membersCount,
            rune_depth: interval.runeDepth,
            start_time: interval.startTime,
            synth_supply: interval.synthSupply,
            synth_units: interval.synthUnits,
            units: interval.units,
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
            "https://midgard.ninerealms.com/v2/history/depths/{}?interval={}&from={}&count=100",
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
