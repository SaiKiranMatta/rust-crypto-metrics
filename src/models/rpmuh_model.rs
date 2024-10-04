use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunePoolHistory {
    pub _id: ObjectId,
    pub start_time: String,
    pub end_time: String,
    pub count: String,
    pub units: String,
}

#[derive(Debug, Deserialize)]
pub struct RunePoolHistoryRequest {
    pub start_time: String,
    pub end_time: String,
    pub count: String,
    pub units: String,
}

impl TryFrom<RunePoolHistoryRequest> for RunePoolHistory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: RunePoolHistoryRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            start_time: item.start_time,
            end_time: item.end_time,
            count: item.count,
            units: item.units,
        })
    }
}
