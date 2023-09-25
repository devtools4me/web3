use wasm_bindgen::JsCast;
use yew::prelude::*;

use algotrader_api::types::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct MarketsProps {
    pub markets: Vec<String>,
}