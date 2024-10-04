use std::{env, str::FromStr, time::SystemTime};

use chrono::Utc;
use dotenv::dotenv;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, from_bson, from_document, oid::ObjectId, to_document, DateTime, Document}, options::FindOptions, results::{InsertOneResult, UpdateResult}, Client, Collection
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
        start_time: Option<i64>,
        end_time: Option<i64>,
        pool: Option<String>,
        page: u32,
        limit: u32,
        sort_by: Option<String>,
        sort_order: i32,
        interval: Option<String>,
    ) -> Result<Vec<Document>, mongodb::error::Error> {
        let mut query = doc! {};
        
        if let Some(pool_value) = pool {
            query.insert("pool", pool_value);
        }
    
        if let Some(from_timestamp) = start_time {
            query.insert("start_time", doc! { "$gte": from_timestamp });
        }
    
        if let Some(to_timestamp) = end_time {
            query.insert("end_time", doc! { "$lte": to_timestamp });
        }
    
        let skip = (page - 1) * limit;
    
        let sort_doc = sort_by.map(|field| {
            let order = if sort_order == 1 { 1 } else { -1 };
            doc! { field: order }
        }).unwrap_or_else(|| doc! {});
    
        let aggregate = match interval.as_deref() {
            Some("day") => Some(86400),
            Some("week") => Some(604800),
            Some("month") => Some(2629743),
            Some("quarter") => Some(7889229),
            Some("year") => Some(31556926),
            _ => None,
        };
    
        if let Some(interval_sec) = aggregate {
            let pipeline = vec![
                doc! {
                    "$match": query
                },
                doc! {
                    "$group": {
                        "_id": {
                            "interval": { 
                                "$dateTrunc": {
                                    "date": { "$toDate": { "$multiply": ["$start_time", 1000] } },
                                    "unit": match interval.as_deref() {
                                        Some("day") => "day",
                                        Some("week") => "week",
                                        Some("month") => "month",
                                        Some("quarter") => "quarter",
                                        Some("year") => "year",
                                        _ => "hour",
                                    }
                                }
                            }
                        },
                        "start_time": { "$min": "$start_time" },
                        "end_time": { "$max": "$end_time" },
                        "asset_depth": { "$avg": "$asset_depth" },
                        "asset_price": { "$avg": "$asset_price" },
                        "asset_price_usd": { "$avg": "$asset_price_usd" },
                        "liquidity_units": { "$sum": "$liquidity_units" },
                        "luvi": { "$avg": "$luvi" },
                        "members_count": { "$max": "$members_count" },
                        "rune_depth": { "$avg": "$rune_depth" },
                        "synth_supply": { "$max": "$synth_supply" },
                        "synth_units": { "$sum": "$synth_units" },
                        "units": { "$sum": "$units" },
                    }
                },
                doc! {
                    "$sort": sort_doc
                },
                doc! {
                    "$skip": skip as i64
                },
                doc! {
                    "$limit": limit as i64
                }
            ];
    
            let mut cursor = self.depth_history.aggregate(pipeline).await?;
            let mut results = Vec::new();
    
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(doc) => {
                        let mut doc = to_document(&doc).unwrap();
                        doc.remove("_id");
                        results.push(doc);
                    },
                    Err(e) => eprintln!("Error parsing document: {:?}", e),
                }
            }
    
            Ok(results)
        } else {
            let mut cursor = self.depth_history
                .find(query)
                .skip(skip as u64)
                .limit(limit as i64)
                .sort(sort_doc)
                .await?;
    
            let mut results = Vec::new();
    
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(doc) => {
                        let mut doc = to_document(&doc).unwrap();
                        doc.remove("_id");
                        results.push(doc);
                    },
                    Err(e) => eprintln!("Error parsing document: {:?}", e),
                }
            }
    
            Ok(results)
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
        start_time: Option<i64>,
        end_time: Option<i64>,
        pool: Option<String>,
        page: u32,
        limit: u32,
        sort_by: Option<String>,
        sort_order: i32,
    ) -> Result<Vec<mongodb::bson::Document>, mongodb::error::Error> {
        let mut query = doc! {};
    
        if let Some(pool_value) = pool {
            query.insert("pool", pool_value);
        }
    
        if let Some(from_timestamp) = start_time {
            query.insert("start_time", doc! { "$gte": from_timestamp });
        }
    
        if let Some(to_timestamp) = end_time {
            query.insert("end_time", doc! { "$lte": to_timestamp });
        }
    
        let skip = (page - 1) * limit;
    
        let sort_doc = sort_by.map(|field| {
            let order = if sort_order == 1 { 1 } else { -1 };
            doc! { field: order }
        }).unwrap_or_else(|| doc! {});
    
        let mut cursor = self.swap_history
            .find(query)
            .skip(skip as u64)
            .limit(limit as i64)
            .sort(sort_doc)
            .await?;
    
        let mut results = Vec::new();
    
        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let mut doc = to_document(&doc).unwrap();
                    doc.remove("_id");
                    results.push(doc);
                },
                Err(e) => eprintln!("Error parsing document: {:?}", e),
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
    
    pub async fn get_rune_pool_history(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        page: u32,
        limit: u32,
        sort_by: Option<String>,
        sort_order: i32,
    ) -> Result<Vec<mongodb::bson::Document>, mongodb::error::Error> {
        let mut query = doc! {};
    
    
        if let Some(from_timestamp) = start_time {
            query.insert("start_time", doc! { "$gte": from_timestamp });
        }
    
        if let Some(to_timestamp) = end_time {
            query.insert("end_time", doc! { "$lte": to_timestamp });
        }
    
        let skip = (page - 1) * limit;
    
        let sort_doc = sort_by.map(|field| {
            let order = if sort_order == 1 { 1 } else { -1 };
            doc! { field: order }
        }).unwrap_or_else(|| doc! {});
    
        let mut cursor = self.rpmuh
            .find(query)
            .skip(skip as u64)
            .limit(limit as i64)
            .sort(sort_doc)
            .await?;
    
        let mut results = Vec::new();
    
        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let mut doc = to_document(&doc).unwrap();
                    doc.remove("_id");
                    results.push(doc);
                },
                Err(e) => eprintln!("Error parsing document: {:?}", e),
            }
        }
    
        Ok(results)
    }
}
