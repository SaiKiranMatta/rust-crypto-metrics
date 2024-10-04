use reqwest::Error;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;

use crate::models::{earning_summary_model::{EarningsSummary, EarningsSummaryRequest}, earnings_model::{PoolEarnings, PoolEarningsRequest}};
use super::db::Database;

#[derive(Debug, Serialize, Deserialize)]
struct Pool {
    pool: String,
    assetLiquidityFees: f64,
    runeLiquidityFees: f64,
    totalLiquidityFeesRune: f64,
    saverEarning: f64,
    rewards: f64,
    earnings: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Interval {
    startTime: String,
    endTime: String,
    avgNodeCount: i64,
    blockRewards: f64,
    bondingEarnings: f64,
    earnings: f64,
    liquidityEarnings: f64,
    liquidityFees: f64,
    runePriceUSD: f64,
    pools: Vec<Pool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    avgNodeCount: i64,
    blockRewards: f64,
    bondingEarnings: f64,
    earnings: f64,
    endTime: String,
    liquidityEarnings: f64,
    liquidityFees: f64,
    pools: Vec<Pool>,
    runePriceUSD: f64,
    startTime: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    intervals: Vec<Interval>,
    meta: Meta,
}

async fn store_earnings_in_db(intervals: Vec<Interval>, meta: Meta, db: &Database) {
    for interval in intervals {
        let summary_request = EarningsSummaryRequest {
            start_time: interval.startTime.clone().parse::<i64>().expect("Failed to parse startTime"),
            end_time: interval.endTime.clone().parse::<i64>().expect("Failed to parse endTime"),
            avg_node_count: interval.avgNodeCount,
            block_rewards: interval.blockRewards,
            bonding_earnings: interval.bondingEarnings,
            liquidity_earnings: interval.liquidityEarnings,
            liquidity_fees: interval.liquidityFees,
            rune_price_usd: interval.runePriceUSD,
        };

        let earnings_summary = EarningsSummary::try_from(summary_request).unwrap();
        let inserted_summary_id = db.create_earnings_summary(earnings_summary).await.unwrap().inserted_id;

        for pool in interval.pools {
            let pool_request = PoolEarningsRequest {
                pool: pool.pool.clone(),
                asset_liquidity_fees: pool.assetLiquidityFees,
                rune_liquidity_fees: pool.runeLiquidityFees,
                total_liquidity_fees_rune: pool.totalLiquidityFeesRune,
                saver_earning: pool.saverEarning,
                rewards: pool.rewards,
                earnings_summary_id: inserted_summary_id.as_object_id().unwrap(),
            };

            let pool_earnings = PoolEarnings::try_from(pool_request).unwrap();
            match db.create_pool_earnings(pool_earnings).await {
                Ok(result) => println!("Successfully inserted pool earnings: {:?}", result.inserted_id),
                Err(e) => eprintln!("Error inserting pool earnings document: {:?}", e),
            }
        }
    }
}

pub async fn fetch_and_store_earnings(db: &Database, interval: &String, start_time: i64) -> Result<(), Error> {
    let mut current_time = start_time;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval={}&from={}&count=400",
            interval,
            current_time
        );

        let api_response = reqwest::get(&url).await?;
        let raw_body = api_response.text().await?;

        println!("Raw response: {}", raw_body);
        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        
        let meta_end_time = response.meta.endTime.clone();
        store_earnings_in_db(response.intervals, response.meta, db).await;

        let end_time: i64 = meta_end_time.parse().unwrap();
        let current_utc: DateTime<Utc> = Utc::now();
        let current_timestamp = current_utc.timestamp();

        if end_time >= current_timestamp {
            break;
        }

        current_time = end_time;
    }

    Ok(())
}
