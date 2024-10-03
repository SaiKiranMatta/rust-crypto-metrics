use std::{env, str::FromStr, time::SystemTime};

use chrono::Utc;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, from_document, oid::ObjectId, DateTime},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::{
    depth_history_model::PoolDepthPriceHistory, 
    earnings_model::PoolEarnings, 
    rpmuh_model::RunePoolHistory, 
    swap_history_model::PoolSwapHistory
};

pub struct Database {
    depth_history: Collection<PoolDepthPriceHistory>,
    earnings: Collection<PoolEarnings>,
    swap_history: Collection<PoolSwapHistory>,
    rpmuh: Collection<RunePoolHistory>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27017/?directConnection=true".to_string(),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("crypto-metrics");

        let depth_history: Collection<PoolDepthPriceHistory> = db.collection("depth_history");
        let earnings: Collection<PoolEarnings> = db.collection("earnings");
        let swap_history: Collection<PoolSwapHistory> = db.collection("swap_history");
        let rpmuh: Collection<RunePoolHistory> = db.collection("rpmuh");

        Database {
            depth_history,
            earnings,
            swap_history,
            rpmuh,
        }
    }

    pub async fn create_depth_history(
        &self,
        depth_history: PoolDepthPriceHistory
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        match self.depth_history.insert_one(depth_history).await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error creating depth history: {:?}", e); 
                Err(e)
            }
        }
    }

    pub async fn create_earnings(
        &self,
        earnings: PoolEarnings
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        match self.earnings.insert_one(earnings).await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error creating earnings: {:?}", e); 
                Err(e)
            }
        }
    }

    pub async fn create_swap_history(
        &self,
        swap_history: PoolSwapHistory
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        match self.swap_history.insert_one(swap_history).await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error creating swap history: {:?}", e); 
                Err(e)
            }
        }
    }

    pub async fn create_rpmuh(
        &self,
        rpmuh: RunePoolHistory
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        match self.rpmuh.insert_one(rpmuh).await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error creating RunePool history: {:?}", e); 
                Err(e)
            }
        }
    }
}
