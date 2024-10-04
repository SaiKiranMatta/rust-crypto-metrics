use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolDepthPriceHistory {
    pub _id: ObjectId,
    pub pool: String, 
    pub asset_depth: f64,
    pub asset_price: f64,
    pub asset_price_usd: f64,
    pub start_time: i64,
    pub end_time: i64,
    pub liquidity_units: f64,
    pub luvi: f64,
    pub members_count: i64,
    pub rune_depth: f64,
    pub synth_supply: f64,
    pub synth_units: f64,
    pub units: f64,
}

#[derive(Debug, Deserialize)]
pub struct PoolDepthPriceHistoryRequest {
    pub pool: String, 
    pub asset_depth: f64,
    pub asset_price: f64,
    pub asset_price_usd: f64,
    pub start_time: i64,
    pub end_time: i64,
    pub liquidity_units: f64,
    pub luvi: f64,
    pub members_count: i64,
    pub rune_depth: f64,
    pub synth_supply: f64,
    pub synth_units: f64,
    pub units: f64,
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
