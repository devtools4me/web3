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

pub type DydxResult<T> = Result<T, String>;

#[async_trait]
pub trait DydxApi {
    async fn get_ohlc_data(&self) -> DydxResult<Vec<Ohlc>>;
}