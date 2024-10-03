use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolEarnings {
    pub _id: ObjectId,
    pub pool: String,
    pub asset_liquidity_fees: String,
    pub rune_liquidity_fees: String,
    pub total_liquidity_fees_rune: String,
    pub saver_earning: String,
    pub rewards: String,
    pub earnings_summary_id: ObjectId,
}

#[derive(Debug, Deserialize)]
pub struct PoolEarningsRequest {
    pub pool: String,
    pub asset_liquidity_fees: String,
    pub rune_liquidity_fees: String,
    pub total_liquidity_fees_rune: String,
    pub saver_earning: String,
    pub rewards: String,
    pub earnings_summary_id: ObjectId,
}

impl TryFrom<PoolEarningsRequest> for PoolEarnings {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: PoolEarningsRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            pool: item.pool,
            asset_liquidity_fees: item.asset_liquidity_fees,
            rune_liquidity_fees: item.rune_liquidity_fees,
            total_liquidity_fees_rune: item.total_liquidity_fees_rune,
            saver_earning: item.saver_earning,
            rewards: item.rewards,
            earnings_summary_id: item.earnings_summary_id,
        })
    }
}
