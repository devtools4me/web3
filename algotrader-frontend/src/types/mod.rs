use wasm_bindgen::JsCast;
use yew::prelude::*;

use algotrader_api::types::*;

pub mod props;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct AppData {
    pub markets: Vec<String>,
}