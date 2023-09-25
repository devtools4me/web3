use wasm_bindgen::JsCast;
use yew::prelude::*;

use algotrader_api::types::*;

#[derive(Properties, PartialEq)]
pub struct StrCbProps {
    pub callback: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct MarketsProps {
    pub markets: Vec<String>,
    pub callback: Callback<String>,
}