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

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait DydxApi {
    async fn get_ohlc_data(&self) -> Result<Vec<Ohlc>>;
}