use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsSummary {
    pub _id: ObjectId,
    pub start_time: i64,               
    pub end_time: i64,                 
    pub block_rewards: f64,            
    pub avg_node_count: f64,           
    pub bonding_earnings: f64,         
    pub liquidity_earnings: f64,       
    pub liquidity_fees: f64,           
    pub rune_price_usd: f64,           
}

#[derive(Debug, Deserialize)]
pub struct EarningsSummaryRequest {
    pub start_time: i64,               
    pub end_time: i64,                 
    pub block_rewards: f64,            
    pub avg_node_count: f64,           
    pub bonding_earnings: f64,         
    pub liquidity_earnings: f64,       
    pub liquidity_fees: f64,           
    pub rune_price_usd: f64,           
}

impl TryFrom<EarningsSummaryRequest> for EarningsSummary {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: EarningsSummaryRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            start_time: item.start_time,
            end_time: item.end_time,
            block_rewards: item.block_rewards,
            avg_node_count: item.avg_node_count,
            bonding_earnings: item.bonding_earnings,
            liquidity_earnings: item.liquidity_earnings,
            liquidity_fees: item.liquidity_fees,
            rune_price_usd: item.rune_price_usd,
        })
    }
}
