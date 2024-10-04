use std::{env, str::FromStr, time::SystemTime};

use chrono::Utc;
use dotenv::dotenv;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, from_document, oid::ObjectId, to_document, DateTime, Document, from_bson,},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::{
    depth_history_model::PoolDepthPriceHistory, earning_summary_model::EarningsSummary, earnings_model::PoolEarnings, rpmuh_model::RunePoolHistory, swap_history_model::PoolSwapHistory
};

pub struct Database {
    depth_history: Collection<PoolDepthPriceHistory>,
    earnings: Collection<PoolEarnings>,
    earnings_summary: Collection<EarningsSummary>,
    swap_history: Collection<PoolSwapHistory>,
    rpmuh: Collection<RunePoolHistory>,
}

impl Database {
    pub async fn init() -> Self {
        dotenv().ok(); 

        let uri = env::var("MONGO_URI").unwrap_or_else(|_| {
            "mongodb://localhost:27017/?directConnection=true".to_string()
        });

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("crypto-metrics");

        let depth_history: Collection<PoolDepthPriceHistory> = db.collection("depth_history");
        let earnings: Collection<PoolEarnings> = db.collection("earnings");
        let earnings_summary: Collection<EarningsSummary> = db.collection("earnings_summary");
        let swap_history: Collection<PoolSwapHistory> = db.collection("swap_history");
        let rpmuh: Collection<RunePoolHistory> = db.collection("rpmuh");

        Database {
            depth_history,
            earnings,
            earnings_summary,
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

    pub async fn get_pool_depth_price_history(
        &self,
        pool: Option<String>,  
        count: Option<u32>,       
        from: Option<i64>,        
        to: Option<i64>,          
    ) -> Result<Vec<mongodb::bson::Document>, mongodb::error::Error> {
        let mut query = doc! {};
        
        if let Some(pool_value) = pool {
            query.insert("pool", pool_value);
        }
    
        if let Some(from_timestamp) = from {
            query.insert("start_time", doc! { "$gte": DateTime::from_millis(from_timestamp * 1000) });
        }
    
        if let Some(to_timestamp) = to {
            query.insert("end_time", doc! { "$lte": DateTime::from_millis(to_timestamp * 1000) });
        }
    
        let mut cursor = self.depth_history.find(query).await?;
    
        let mut results = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(mut doc) => {
                    let mut doc = to_document(&doc).unwrap();
                    doc.remove("_id");
                    doc.remove("pool");
                    results.push(doc);
                },
                Err(e) => eprintln!("Error parsing document: {:?}", e),
            }
            if let Some(c) = count {
                if results.len() as u32 >= c {
                    break;
                }
            }
        }
    
        Ok(results)
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

    pub async fn create_pool_earnings(
        &self,
        pool_earnings: PoolEarnings
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        match self.earnings.insert_one(pool_earnings).await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error creating pool earnings: {:?}", e); 
                Err(e)
            }
        }
    }

    pub async fn create_earnings_summary(
        &self,
        earnings_summary: EarningsSummary
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        match self.earnings_summary.insert_one(earnings_summary).await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error creating earnings summary: {:?}", e); 
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

    pub async fn get_pool_swap_history(
        &self,
        pool: Option<String>,
        count: Option<u32>,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<mongodb::bson::Document>, mongodb::error::Error> {
        let mut query = doc! {};
        
        if let Some(pool_value) = pool {
            query.insert("pool", pool_value);
        }
    
        if let Some(from_timestamp) = from {
            query.insert("start_time", doc! { "$gte": DateTime::from_millis(from_timestamp * 1000) });
        }
    
        if let Some(to_timestamp) = to {
            query.insert("end_time", doc! { "$lte": DateTime::from_millis(to_timestamp * 1000) });
        }
    
        let mut cursor = self.swap_history.find(query).await?;
    
        let mut results = Vec::new();
    
        while let Some(result) = cursor.next().await {
            match result {
                Ok(mut doc) => {
                    let mut doc = to_document(&doc).unwrap();
                    doc.remove("_id");
                    results.push(doc);
                },
                Err(e) => eprintln!("Error parsing document: {:?}", e),
            }
    
            if let Some(c) = count {
                if results.len() as u32 >= c {
                    break;
                }
            }
        }
    
        Ok(results)
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
