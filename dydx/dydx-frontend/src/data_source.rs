use dydx_api::types::*;

use crate::mock::{*, MockDydxApi};
use crate::gloonet::GlooNetDydxApi;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static CLIENT: MockDydxApi = MockDydxApi {};
//static CLIENT: GlooNetDydxApi = GlooNetDydxApi {};

pub fn get_data() -> Result<Vec<Ohlc>> {
    let response = CLIENT.get_ohlc_data();
    Ok(response)
}