use std::{env, str::FromStr, time::SystemTime};

use chrono::Utc;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, from_document, oid::ObjectId, DateTime},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::{depth_history_model::PoolDepthPriceHistory, earnings_model::PoolEarnings, rpmuh_model::RunePoolHistory, swap_history_model::PoolSwapHistory};



pub struct Database {
    depth_history: Collection<PoolDepthPriceHistory>,
    earnings: Collection<PoolEarnings>,
    swap_history: Collection<PoolSwapHistory>,
    rpmuh: Collection<RunePoolHistory>

}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27017/?directConnection=true".to_string(),
        };

        // let uri = "".to_string();
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
            rpmuh
        }
    }

}