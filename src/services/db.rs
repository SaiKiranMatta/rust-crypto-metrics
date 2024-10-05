use std::env;


use dotenv::dotenv;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document,  Document}, results::InsertOneResult, Client, Collection
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
        }).unwrap_or_else(|| doc! { "end_time": -1 });  
    
        let interval_unit = interval.as_deref().unwrap_or("hour");
    
        if interval_unit == "hour" {
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
    
            return Ok(results);
        }
    
        let interval_duration = match interval_unit {
            "day" => 86400,
            "week" => 604800,
            "month" => 2678400, 
            "quarter" => 7948800, 
            "year" => 31622400,  
            _ => 3600, 
        };
    
        let pipeline = vec![
            doc! { "$match": query },
    
            doc! { "$group": {
                "_id": {
                    // Subtracting 1ms to include the last element as well
                   "interval_start": { 
                        "$subtract": [ 
                            { "$add": ["$end_time", 1] }, 
                            { "$mod": [ 
                                { "$subtract": ["$end_time", 1] },  
                                interval_duration 
                            ] }
                        ]
                    }
                },
                "last_entry": { "$last": "$$ROOT" }  
            }},
            
            doc! { "$project": {
                "_id": 0,
                "pool": "$last_entry.pool",
                "asset_depth": "$last_entry.asset_depth",
                "asset_price": "$last_entry.asset_price",
                "asset_price_usd": "$last_entry.asset_price_usd",
                "liquidity_units": "$last_entry.liquidity_units",
                "luvi": "$last_entry.luvi",
                "members_count": "$last_entry.members_count",
                "rune_depth": "$last_entry.rune_depth",
                "synth_supply": "$last_entry.synth_supply",
                "synth_units": "$last_entry.synth_units",
                "units": "$last_entry.units",
                "start_time": {
                    "$subtract": [ "$last_entry.start_time", { "$mod": [ "$last_entry.start_time", interval_duration ] }]
                },
                "end_time": {
                    "$add": [
                        { "$subtract": [ "$last_entry.start_time", { "$mod": [ "$last_entry.start_time", interval_duration ] }] },
                        interval_duration
                    ]
                }
            }},
            
            doc! { "$sort": sort_doc },
    
            doc! { "$skip": skip as i64 },
            doc! { "$limit": limit as i64 },
        ];
    
        let mut cursor = self.depth_history.aggregate(pipeline).await?;
        let mut results = Vec::new();
    
        while let Some(result) = cursor.next().await {
            match result {
                Ok(mut doc) => {
                    doc.remove("_id");  
                    results.push(doc);  
                },
                Err(e) => eprintln!("Error parsing document: {:?}", e),
            }
        }
    
        Ok(results)
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



    pub async fn get_pool_earnings(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        pool: Option<String>,
        page: u32,
        limit: u32,
        sort_by: Option<String>,
        sort_order: i32,
        interval: Option<String>, 
        include_summary: bool,
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
        }).unwrap_or_else(|| doc! { "end_time": -1 });
    
       
        let interval_unit = interval.as_deref().unwrap_or("hour");
    
        
        if interval_unit == "hour" {
            let mut cursor = self.earnings
                .find(query)
                .skip(skip as u64)
                .limit(limit as i64)
                .sort(sort_doc)
                .await?;
    
            let mut results = Vec::new();
    
            while let Some(result) = cursor.next().await {
                match result {
                    Ok( doc) => {
                        let mut doc = to_document(&doc).unwrap();
                        doc.remove("_id");
    
                        if include_summary {
                            if let Some(earnings_summary_id) = doc.get_object_id("earnings_summary_id").ok() {
                                if let Ok(Some(summary_doc)) = self.get_earnings_summary(earnings_summary_id).await {
                                    for (key, value) in summary_doc.iter() {
                                        doc.insert(key.clone(), value.clone());
                                    }
                                }
                            }
                        }
                        doc.remove("earnings_summary_id");
                        results.push(doc);
                    },
                    Err(e) => eprintln!("Error parsing document: {:?}", e),
                }
            }
    
            return Ok(results);
        }
    
        let interval_duration = match interval_unit {
            "day" => 86400,
            "week" => 604800,
            "month" => 2678400, 
            "quarter" => 7948800, 
            "year" => 31622400,  
            _ => 3600, 
        };
    
        
        let pipeline = vec![
            doc! { "$match": query },
    
            doc! { "$group": {
                "_id": {
                    "interval_start": { 
                        "$subtract": [ 
                            { "$add": ["$end_time", 1] }, 
                            { "$mod": [ 
                                { "$subtract": ["$end_time", 1] },  
                                interval_duration 
                            ] }
                        ]
                    }
                },
                "last_entry": { "$last": "$$ROOT" }  
            }},
            
            doc! { "$project": {
                "_id": 0,
                "pool": "$last_entry.pool",
                "asset_liquidity_fees": "$last_entry.asset_liquidity_fees",
                "rune_liquidity_fees": "$last_entry.rune_liquidity_fees",
                "total_liquidity_fees_rune": "$last_entry.total_liquidity_fees_rune",
                "saver_earning": "$last_entry.saver_earning",
                "rewards": "$last_entry.rewards",
                "earnings_summary_id": "$last_entry.earnings_summary_id",
                "start_time": {
                    "$subtract": [ "$last_entry.start_time", { "$mod": [ "$last_entry.start_time", interval_duration ] }]
                },
                "end_time": {
                    "$add": [
                        { "$subtract": [ "$last_entry.start_time", { "$mod": [ "$last_entry.start_time", interval_duration ] }] },
                        interval_duration
                    ]
                }
            }},
            
            doc! { "$sort": sort_doc },
    
            doc! { "$skip": skip as i64 },
            doc! { "$limit": limit as i64 },
        ];
    

    
        let mut cursor = self.earnings.aggregate(pipeline).await?;
        let mut results = Vec::new();
    
        while let Some(result) = cursor.next().await {
            match result {
                Ok( doc) => {
                    let mut doc = to_document(&doc).unwrap();
                    doc.remove("_id");

                    if include_summary {
                        if let Some(earnings_summary_id) = doc.get_object_id("earnings_summary_id").ok() {
                            if let Ok(Some(summary_doc)) = self.get_earnings_summary(earnings_summary_id).await {
                                for (key, value) in summary_doc.iter() {
                                    doc.insert(key.clone(), value.clone());
                                }
                            }
                        }
                    }
                    doc.remove("earnings_summary_id");
                    results.push(doc);
                },
                Err(e) => eprintln!("Error parsing document: {:?}", e),
            }
        }
    
        Ok(results)
    }
    
    pub async fn get_earnings_summary(
        &self,
        earnings_summary_id: ObjectId,
    ) -> Result<Option<Document>, mongodb::error::Error> {
        let filter = doc! { "_id": earnings_summary_id };
    
        match self.earnings_summary.find_one(filter).await {
            Ok(Some(summary_doc)) => {
                let mut summary = to_document(&summary_doc).unwrap();
                summary.remove("_id"); 
                summary.remove("start_time");
                summary.remove("end_time");
                Ok(Some(summary))
            }
            Ok(None) => {
                Ok(None)
            }
            Err(err) => {
                Err(err)
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
        interval: Option<String>,
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
    
        let interval_unit = interval.as_deref().unwrap_or("hour");
    
        if interval_unit == "hour" {
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
    
            return Ok(results);
        }
    
        let interval_duration = match interval_unit {
            "day" => 86400,
            "week" => 604800,
            "month" => 2678400,
            "quarter" => 7948800,
            "year" => 31622400,
            _ => 3600,
        };
    
        let pipeline = vec![
            doc! { "$match": query },
    
            doc! { "$group": {
                "_id": {
                    "interval_start": { 
                        "$subtract": [ 
                            { "$add": ["$end_time", 1] }, 
                            { "$mod": [ 
                                { "$subtract": ["$end_time", 1] },  
                                interval_duration 
                            ] }
                        ]
                    }
                },
                "last_entry": { "$last": "$$ROOT" }  
            }},
            
            doc! { "$project": {
                "_id": "$last_entry._id",
                "pool": "$last_entry.pool",
                "start_time": {
                    "$subtract": [ "$last_entry.start_time", { "$mod": [ "$last_entry.start_time", interval_duration ] }]
                },
                "end_time": {
                    "$add": [
                        { "$subtract": [ "$last_entry.start_time", { "$mod": [ "$last_entry.start_time", interval_duration ] }] },
                        interval_duration
                    ]
                },
                "to_asset_count": "$last_entry.to_asset_count",
                "to_rune_count": "$last_entry.to_rune_count",
                "to_trade_count": "$last_entry.to_trade_count",
                "from_trade_count": "$last_entry.from_trade_count",
                "synth_mint_count": "$last_entry.synth_mint_count",
                "synth_redeem_count": "$last_entry.synth_redeem_count",
                "total_count": "$last_entry.total_count",
                "to_asset_volume": "$last_entry.to_asset_volume",
                "to_rune_volume": "$last_entry.to_rune_volume",
                "to_trade_volume": "$last_entry.to_trade_volume",
                "from_trade_volume": "$last_entry.from_trade_volume",
                "synth_mint_volume": "$last_entry.synth_mint_volume",
                "synth_redeem_volume": "$last_entry.synth_redeem_volume",
                "total_volume": "$last_entry.total_volume",
                "to_asset_volume_usd": "$last_entry.to_asset_volume_usd",
                "to_rune_volume_usd": "$last_entry.to_rune_volume_usd",
                "to_trade_volume_usd": "$last_entry.to_trade_volume_usd",
                "from_trade_volume_usd": "$last_entry.from_trade_volume_usd",
                "synth_mint_volume_usd": "$last_entry.synth_mint_volume_usd",
                "synth_redeem_volume_usd": "$last_entry.synth_redeem_volume_usd",
                "total_volume_usd": "$last_entry.total_volume_usd",
                "to_asset_fees": "$last_entry.to_asset_fees",
                "to_rune_fees": "$last_entry.to_rune_fees",
                "to_trade_fees": "$last_entry.to_trade_fees",
                "from_trade_fees": "$last_entry.from_trade_fees",
                "synth_mint_fees": "$last_entry.synth_mint_fees",
                "synth_redeem_fees": "$last_entry.synth_redeem_fees",
                "total_fees": "$last_entry.total_fees",
                "to_asset_average_slip": "$last_entry.to_asset_average_slip",
                "to_rune_average_slip": "$last_entry.to_rune_average_slip",
                "to_trade_average_slip": "$last_entry.to_trade_average_slip",
                "from_trade_average_slip": "$last_entry.from_trade_average_slip",
                "synth_mint_average_slip": "$last_entry.synth_mint_average_slip",
                "synth_redeem_average_slip": "$last_entry.synth_redeem_average_slip",
                "average_slip": "$last_entry.average_slip",
                "rune_price_usd": "$last_entry.rune_price_usd"
            }},
            
            doc! { "$sort": sort_doc },
    
            doc! { "$skip": skip as i64 },
            doc! { "$limit": limit as i64 },
        ];
    
        let mut cursor = self.swap_history.aggregate(pipeline).await?;
        let mut results = Vec::new();
    
        while let Some(result) = cursor.next().await {
            match result {
                Ok(mut doc) => {
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
        interval: Option<String>,
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
    
        let interval_unit = interval.as_deref().unwrap_or("hour");
    
        if interval_unit == "hour" {
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
    
            return Ok(results);
        }
    
        let interval_duration = match interval_unit {
            "day" => 86400,
            "week" => 604800,
            "month" => 2678400, 
            "quarter" => 7948800, 
            "year" => 31622400, 
            _ => 3600,
        };
    
        let pipeline = vec![
            doc! { "$match": query },
    
            doc! { "$group": {
                "_id": {
                    "interval_start": { 
                        "$subtract": [ 
                            { "$add": ["$end_time", 1] }, 
                            { "$mod": [ 
                                { "$subtract": ["$end_time", 1] },  
                                interval_duration 
                            ] }
                        ]
                    }
                },
                "last_entry": { "$last": "$$ROOT" }  
            }},
            
            doc! { "$project": {
                "start_time": {
                    "$subtract": [ "$last_entry.start_time", { "$mod": [ "$last_entry.start_time", interval_duration ] }]
                },
                "end_time": {
                    "$add": [
                        { "$subtract": [ "$last_entry.start_time", { "$mod": [ "$last_entry.start_time", interval_duration ] }] },
                        interval_duration
                    ]
                },
                "count": "$last_entry.count",
                "units": "$last_entry.units"
            }},
            
            doc! { "$sort": sort_doc },
    
            doc! { "$skip": skip as i64 },
            doc! { "$limit": limit as i64 },
        ];
    
        let mut cursor = self.rpmuh.aggregate(pipeline).await?;
        let mut results = Vec::new();
    
        while let Some(result) = cursor.next().await {
            match result {
                Ok(mut doc) => {
                    doc.remove("_id");
                    results.push(doc);
                },
                Err(e) => eprintln!("Error parsing document: {:?}", e),
            }
        }
    
        Ok(results)
    }
    
}
