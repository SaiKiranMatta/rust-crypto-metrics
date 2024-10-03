use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolEarnings {
    pub _id: ObjectId,
    pub pool: String, // e.g., "BTC.BTC", "ETH.ETH"
    pub start_time: String,
    pub end_time: String,
    pub liquidity_fees: String,
    pub block_rewards: String,
    pub earnings: String,
    pub bonding_earnings: String,
    pub liquidity_earnings: String,
    pub avg_node_count: String,
    pub rune_price_usd: String,
    pub asset_liquidity_fees: String,
    pub rune_liquidity_fees: String,
    pub total_liquidity_fees_rune: String,
    pub saver_earning: String,
    pub rewards: String,
}

#[derive(Debug, Deserialize)]
pub struct PoolEarningsRequest {
    pub pool: String,
    pub start_time: String,
    pub end_time: String,
    pub liquidity_fees: String,
    pub block_rewards: String,
    pub earnings: String,
    pub bonding_earnings: String,
    pub liquidity_earnings: String,
    pub avg_node_count: String,
    pub rune_price_usd: String,
    pub asset_liquidity_fees: String,
    pub rune_liquidity_fees: String,
    pub total_liquidity_fees_rune: String,
    pub saver_earning: String,
    pub rewards: String,
}

impl TryFrom<PoolEarningsRequest> for PoolEarnings {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: PoolEarningsRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            pool: item.pool,
            start_time: item.start_time,
            end_time: item.end_time,
            liquidity_fees: item.liquidity_fees,
            block_rewards: item.block_rewards,
            earnings: item.earnings,
            bonding_earnings: item.bonding_earnings,
            liquidity_earnings: item.liquidity_earnings,
            avg_node_count: item.avg_node_count,
            rune_price_usd: item.rune_price_usd,
            asset_liquidity_fees: item.asset_liquidity_fees,
            rune_liquidity_fees: item.rune_liquidity_fees,
            total_liquidity_fees_rune: item.total_liquidity_fees_rune,
            saver_earning: item.saver_earning,
            rewards: item.rewards,
        })
    }
}
