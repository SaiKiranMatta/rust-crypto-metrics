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
            pool: "BTC.BTC".to_string(), // Update this to the actual pool being used
            start_time: interval.startTime,
            end_time: interval.endTime,
            to_asset_count: interval.toAssetCount,
            to_rune_count: interval.toRuneCount,
            to_trade_count: interval.toTradeCount,
            from_trade_count: interval.fromTradeCount,
            synth_mint_count: interval.synthMintCount,
            synth_redeem_count: interval.synthRedeemCount,
            total_count: interval.totalCount,
            to_asset_volume: interval.toAssetVolume,
            to_rune_volume: interval.toRuneVolume,
            to_trade_volume: interval.toTradeVolume,
            from_trade_volume: interval.fromTradeVolume,
            synth_mint_volume: interval.synthMintVolume,
            synth_redeem_volume: interval.synthRedeemVolume,
            total_volume: interval.totalVolume,
            to_asset_volume_usd: interval.toAssetVolumeUSD,
            to_rune_volume_usd: interval.toRuneVolumeUSD,
            to_trade_volume_usd: interval.toTradeVolumeUSD,
            from_trade_volume_usd: interval.fromTradeVolumeUSD,
            synth_mint_volume_usd: interval.synthMintVolumeUSD,
            synth_redeem_volume_usd: interval.synthRedeemVolumeUSD,
            total_volume_usd: interval.totalVolumeUSD,
            to_asset_fees: interval.toAssetFees,
            to_rune_fees: interval.toRuneFees,
            to_trade_fees: interval.toTradeFees,
            from_trade_fees: interval.fromTradeFees,
            synth_mint_fees: interval.synthMintFees,
            synth_redeem_fees: interval.synthRedeemFees,
            total_fees: interval.totalFees,
            to_asset_average_slip: interval.toAssetAverageSlip,
            to_rune_average_slip: interval.toRuneAverageSlip,
            to_trade_average_slip: interval.toTradeAverageSlip,
            from_trade_average_slip: interval.fromTradeAverageSlip,
            synth_mint_average_slip: interval.synthMintAverageSlip,
            synth_redeem_average_slip: interval.synthRedeemAverageSlip,
            average_slip: interval.averageSlip,
            rune_price_usd: interval.runePriceUSD,
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
            "https://midgard.ninerealms.com/v2/history/swaps/{}?interval={}&from={}&count=100",
            pool,
            interval,
            current_time
        );

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
