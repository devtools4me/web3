use dydx_api::types::*;

use crate::mock::{*, MockDydxApi};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_data() -> Result<Vec<Ohlc>> {
    let client = MockDydxApi {};
    let res = client.get_ohlc_data();
    Ok(res)
}