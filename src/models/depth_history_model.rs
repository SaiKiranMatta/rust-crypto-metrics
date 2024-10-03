use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolDepthPriceHistory {
    pub _id: ObjectId,
    pub pool: String, // e.g., "BTC.BTC", "ETH.ETH"
    pub asset_depth: String,
    pub asset_price: String,
    pub asset_price_usd: String,
    pub start_time: String,
    pub end_time: String,
    pub liquidity_units: String,
    pub luvi: String,
    pub members_count: String,
    pub rune_depth: String,
    pub synth_supply: String,
    pub synth_units: String,
    pub units: String,
}

#[derive(Debug, Deserialize)]
pub struct PoolDepthPriceHistoryRequest {
    pub pool: String,
    pub asset_depth: String,
    pub asset_price: String,
    pub asset_price_usd: String,
    pub start_time: String,
    pub end_time: String,
    pub liquidity_units: String,
    pub luvi: String,
    pub members_count: String,
    pub rune_depth: String,
    pub synth_supply: String,
    pub synth_units: String,
    pub units: String,
}

impl TryFrom<PoolDepthPriceHistoryRequest> for PoolDepthPriceHistory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: PoolDepthPriceHistoryRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            pool: item.pool,
            asset_depth: item.asset_depth,
            asset_price: item.asset_price,
            asset_price_usd: item.asset_price_usd,
            start_time: item.start_time,
            end_time: item.end_time,
            liquidity_units: item.liquidity_units,
            luvi: item.luvi,
            members_count: item.members_count,
            rune_depth: item.rune_depth,
            synth_supply: item.synth_supply,
            synth_units: item.synth_units,
            units: item.units,
        })
    }
}
