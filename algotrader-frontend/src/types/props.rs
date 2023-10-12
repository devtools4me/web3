use wasm_bindgen::JsCast;
use yew::prelude::*;

use algotrader_api::types::*;

#[derive(Properties, PartialEq)]
pub struct OhlcListProps {
    pub ohlc_data: Vec<Ohlc>,
}

#[derive(Properties, PartialEq)]
pub struct OhlcProps {
    pub market: String,
    pub resolution: String
}

#[derive(Properties, PartialEq)]
pub struct StrCbProps {
    pub callback: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct StrCbPairProps {
    pub callback1: Callback<String>,
    pub callback2: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct MarketsProps {
    pub markets: Vec<String>,
    pub callback: Callback<String>,
}