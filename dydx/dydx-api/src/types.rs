use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OhlcResponse {
    pub list: Vec<Ohlc>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Ohlc {
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeseriesResponse {
    pub list: Vec<Timeseries>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Timeseries {
    pub value: String,
    pub timestamp: String,
}