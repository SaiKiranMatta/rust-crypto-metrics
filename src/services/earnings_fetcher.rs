use reqwest::Error;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;

use crate::models::{earning_summary_model::{EarningsSummary, EarningsSummaryRequest}, earnings_model::{PoolEarnings, PoolEarningsRequest}};
use super::db::Database;

#[derive(Debug, Serialize, Deserialize)]
struct Pool {
    pool: String,
    assetLiquidityFees: String,
    runeLiquidityFees: String,
    totalLiquidityFeesRune: String,
    saverEarning: String,
    rewards: String,
    earnings: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Interval {
    startTime: String,
    endTime: String,
    avgNodeCount: String,
    blockRewards: String,
    bondingEarnings: String,
    earnings: String,
    liquidityEarnings: String,
    liquidityFees: String,
    runePriceUSD: String,
    pools: Vec<Pool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    avgNodeCount: String,
    blockRewards: String,
    bondingEarnings: String,
    earnings: String,
    endTime: String,
    liquidityEarnings: String,
    liquidityFees: String,
    pools: Vec<Pool>,
    runePriceUSD: String,
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
            start_time: interval.startTime.clone(),
            end_time: interval.endTime.clone(),
            avg_node_count: interval.avgNodeCount.clone(),
            block_rewards: interval.blockRewards.clone(),
            bonding_earnings: interval.bondingEarnings.clone(),
            liquidity_earnings: interval.liquidityEarnings.clone(),
            liquidity_fees: interval.liquidityFees.clone(),
            rune_price_usd: interval.runePriceUSD.clone(),
        };

        let earnings_summary = EarningsSummary::try_from(summary_request).unwrap();
        let inserted_summary_id = db.create_earnings_summary(earnings_summary).await.unwrap().inserted_id;

        for pool in interval.pools {
            let pool_request = PoolEarningsRequest {
                pool: pool.pool.clone(),
                asset_liquidity_fees: pool.assetLiquidityFees.clone(),
                rune_liquidity_fees: pool.runeLiquidityFees.clone(),
                total_liquidity_fees_rune: pool.totalLiquidityFeesRune.clone(),
                saver_earning: pool.saverEarning.clone(),
                rewards: pool.rewards.clone(),
                earnings_summary_id: inserted_summary_id.as_object_id().unwrap(),
            };

            let pool_earnings = PoolEarnings::try_from(pool_request).unwrap();
            match db.create_pool_earnings(pool_earnings).await {
                Ok(result) => println!("Successfully inserted pool earnings: {:?}", result.inserted_id),
                Err(e) => eprintln!("Error inserting pool earnings document: {:?}", e),
            }
        }
    }

    let meta_summary_request = EarningsSummaryRequest {
        start_time: meta.startTime.clone(),
        end_time: meta.endTime.clone(),
        avg_node_count: meta.avgNodeCount.clone(),
        block_rewards: meta.blockRewards.clone(),
        bonding_earnings: meta.bondingEarnings.clone(),
        liquidity_earnings: meta.liquidityEarnings.clone(),
        liquidity_fees: meta.liquidityFees.clone(),
        rune_price_usd: meta.runePriceUSD.clone(),
    };

    let meta_earnings_summary = EarningsSummary::try_from(meta_summary_request).unwrap();
    db.create_earnings_summary(meta_earnings_summary).await.unwrap();
}

pub async fn fetch_and_store_earnings(db: &Database, interval: &String, start_time: i64) -> Result<(), Error> {
    let mut current_time = start_time;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval={}&from={}&count=300",
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
