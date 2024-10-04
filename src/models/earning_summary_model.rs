use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsSummary {
    pub _id: ObjectId,
    pub start_time: i64,         
    pub end_time: i64, 
    pub block_rewards: String,           
    pub avg_node_count: String,       
    pub bonding_earnings: String,    
    pub liquidity_earnings: String,  
    pub liquidity_fees: String,     
    pub rune_price_usd: String,      
}

#[derive(Debug, Deserialize)]
pub struct EarningsSummaryRequest {
    pub start_time: i64,
    pub end_time: i64,
    pub avg_node_count: String,
    pub block_rewards: String,
    pub bonding_earnings: String,
    pub liquidity_earnings: String,
    pub liquidity_fees: String,
    pub rune_price_usd: String,
}

impl TryFrom<EarningsSummaryRequest> for EarningsSummary {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: EarningsSummaryRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            start_time: item.start_time,
            end_time: item.end_time,
            avg_node_count: item.avg_node_count,
            block_rewards: item.block_rewards,
            bonding_earnings: item.bonding_earnings,
            liquidity_earnings: item.liquidity_earnings,
            liquidity_fees: item.liquidity_fees,
            rune_price_usd: item.rune_price_usd,
        })
    }
}
