use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub fn candles_url<'a>(market: &'a str, resolution: &'a str) -> String {
    format!("/candles/{}/{}", market, resolution)
}

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