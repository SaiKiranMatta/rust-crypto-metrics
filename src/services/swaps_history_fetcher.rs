use reqwest::Error;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


use crate::models::swap_history_model::{PoolSwapHistory, PoolSwapHistoryRequest};

use super::db::Database;

#[derive(Debug, Serialize, Deserialize)]
struct SwapsMeta {
    startTime: String,
    endTime: String,
    toAssetCount: String,
    toRuneCount: String,
    toTradeCount: String,
    fromTradeCount: String,
    synthMintCount: String,
    synthRedeemCount: String,
    totalCount: String,
    toAssetVolume: String,
    toRuneVolume: String,
    toTradeVolume: String,
    fromTradeVolume: String,
    synthMintVolume: String,
    synthRedeemVolume: String,
    totalVolume: String,
    toAssetVolumeUSD: String,
    toRuneVolumeUSD: String,
    toTradeVolumeUSD: String,
    fromTradeVolumeUSD: String,
    synthMintVolumeUSD: String,
    synthRedeemVolumeUSD: String,
    totalVolumeUSD: String,
    toAssetFees: String,
    toRuneFees: String,
    toTradeFees: String,
    fromTradeFees: String,
    synthMintFees: String,
    synthRedeemFees: String,
    totalFees: String,
    toAssetAverageSlip: String,
    toRuneAverageSlip: String,
    toTradeAverageSlip: String,
    fromTradeAverageSlip: String,
    synthMintAverageSlip: String,
    synthRedeemAverageSlip: String,
    averageSlip: String,
    runePriceUSD: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SwapsInterval {
    startTime: String,
    endTime: String,
    toAssetCount: String,
    toRuneCount: String,
    toTradeCount: String,
    fromTradeCount: String,
    synthMintCount: String,
    synthRedeemCount: String,
    totalCount: String,
    toAssetVolume: String,
    toRuneVolume: String,
    toTradeVolume: String,
    fromTradeVolume: String,
    synthMintVolume: String,
    synthRedeemVolume: String,
    totalVolume: String,
    toAssetVolumeUSD: String,
    toRuneVolumeUSD: String,
    toTradeVolumeUSD: String,
    fromTradeVolumeUSD: String,
    synthMintVolumeUSD: String,
    synthRedeemVolumeUSD: String,
    totalVolumeUSD: String,
    toAssetFees: String,
    toRuneFees: String,
    toTradeFees: String,
    fromTradeFees: String,
    synthMintFees: String,
    synthRedeemFees: String,
    totalFees: String,
    toAssetAverageSlip: String,
    toRuneAverageSlip: String,
    toTradeAverageSlip: String,
    fromTradeAverageSlip: String,
    synthMintAverageSlip: String,
    synthRedeemAverageSlip: String,
    averageSlip: String,
    runePriceUSD: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SwapsApiResponse {
    meta: SwapsMeta,
    intervals: Vec<SwapsInterval>,
}

async fn store_swaps_intervals_in_db(intervals: Vec<SwapsInterval>, db: &Database) {
    for interval in intervals {
        let new_swap_history = PoolSwapHistory::try_from(PoolSwapHistoryRequest {
            pool: "BTC.BTC".to_string(), // Assuming the pool is always a valid string
            start_time: interval.startTime.parse::<i64>().expect("Failed to parse startTime"),
            end_time: interval.endTime.parse::<i64>().expect("Failed to parse endTime"),
            
            to_asset_count: interval.toAssetCount.parse::<i64>().expect("Failed to parse toAssetCount"),
            to_rune_count: interval.toRuneCount.parse::<i64>().expect("Failed to parse toRuneCount"),
            to_trade_count: interval.toTradeCount.parse::<i64>().expect("Failed to parse toTradeCount"),
            from_trade_count: interval.fromTradeCount.parse::<i64>().expect("Failed to parse fromTradeCount"),
            synth_mint_count: interval.synthMintCount.parse::<i64>().expect("Failed to parse synthMintCount"),
            synth_redeem_count: interval.synthRedeemCount.parse::<i64>().expect("Failed to parse synthRedeemCount"),
            total_count: interval.totalCount.parse::<i64>().expect("Failed to parse totalCount"),
            
            to_asset_volume: interval.toAssetVolume.parse::<f64>().expect("Failed to parse toAssetVolume"),
            to_rune_volume: interval.toRuneVolume.parse::<f64>().expect("Failed to parse toRuneVolume"),
            to_trade_volume: interval.toTradeVolume.parse::<f64>().expect("Failed to parse toTradeVolume"),
            from_trade_volume: interval.fromTradeVolume.parse::<f64>().expect("Failed to parse fromTradeVolume"),
            synth_mint_volume: interval.synthMintVolume.parse::<f64>().expect("Failed to parse synthMintVolume"),
            synth_redeem_volume: interval.synthRedeemVolume.parse::<f64>().expect("Failed to parse synthRedeemVolume"),
            total_volume: interval.totalVolume.parse::<f64>().expect("Failed to parse totalVolume"),
            
            to_asset_volume_usd: interval.toAssetVolumeUSD.parse::<f64>().expect("Failed to parse toAssetVolumeUSD"),
            to_rune_volume_usd: interval.toRuneVolumeUSD.parse::<f64>().expect("Failed to parse toRuneVolumeUSD"),
            to_trade_volume_usd: interval.toTradeVolumeUSD.parse::<f64>().expect("Failed to parse toTradeVolumeUSD"),
            from_trade_volume_usd: interval.fromTradeVolumeUSD.parse::<f64>().expect("Failed to parse fromTradeVolumeUSD"),
            synth_mint_volume_usd: interval.synthMintVolumeUSD.parse::<f64>().expect("Failed to parse synthMintVolumeUSD"),
            synth_redeem_volume_usd: interval.synthRedeemVolumeUSD.parse::<f64>().expect("Failed to parse synthRedeemVolumeUSD"),
            total_volume_usd: interval.totalVolumeUSD.parse::<f64>().expect("Failed to parse totalVolumeUSD"),
            
            to_asset_fees: interval.toAssetFees.parse::<f64>().expect("Failed to parse toAssetFees"),
            to_rune_fees: interval.toRuneFees.parse::<f64>().expect("Failed to parse toRuneFees"),
            to_trade_fees: interval.toTradeFees.parse::<f64>().expect("Failed to parse toTradeFees"),
            from_trade_fees: interval.fromTradeFees.parse::<f64>().expect("Failed to parse fromTradeFees"),
            synth_mint_fees: interval.synthMintFees.parse::<f64>().expect("Failed to parse synthMintFees"),
            synth_redeem_fees: interval.synthRedeemFees.parse::<f64>().expect("Failed to parse synthRedeemFees"),
            total_fees: interval.totalFees.parse::<f64>().expect("Failed to parse totalFees"),
            
            to_asset_average_slip: interval.toAssetAverageSlip.parse::<f64>().expect("Failed to parse toAssetAverageSlip"),
            to_rune_average_slip: interval.toRuneAverageSlip.parse::<f64>().expect("Failed to parse toRuneAverageSlip"),
            to_trade_average_slip: interval.toTradeAverageSlip.parse::<f64>().expect("Failed to parse toTradeAverageSlip"),
            from_trade_average_slip: interval.fromTradeAverageSlip.parse::<f64>().expect("Failed to parse fromTradeAverageSlip"),
            synth_mint_average_slip: interval.synthMintAverageSlip.parse::<f64>().expect("Failed to parse synthMintAverageSlip"),
            synth_redeem_average_slip: interval.synthRedeemAverageSlip.parse::<f64>().expect("Failed to parse synthRedeemAverageSlip"),
            average_slip: interval.averageSlip.parse::<f64>().expect("Failed to parse averageSlip"),
            
            rune_price_usd: interval.runePriceUSD.parse::<f64>().expect("Failed to parse runePriceUSD"),
        })
        .unwrap();
        

        match db.create_swap_history(new_swap_history).await {
            Ok(result) => println!("Successfully inserted: {:?}", result.inserted_id),
            Err(e) => eprintln!("Error inserting document: {:?}", e),
        }
    }
}

pub async fn fetch_and_store_swaps_history(db: &Database, pool: &String, interval: &String, start_time: i64) -> Result<(), Error> {
    let mut current_time = start_time;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?pool={}&interval={}&from={}&count=400",
            pool,
            interval,
            current_time
        );


        let api_response = reqwest::get(&url).await?;
        let raw_body = api_response.text().await?;

        println!("Raw response: {}", raw_body);

        let response = reqwest::get(&url).await?.json::<SwapsApiResponse>().await?;

        store_swaps_intervals_in_db(response.intervals, db).await;
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
