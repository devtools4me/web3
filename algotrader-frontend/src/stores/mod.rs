use yewdux::prelude::*;

use algotrader_common::utils::env_utils;

#[derive(Clone, PartialEq, Eq, Store)]
pub struct AppStore {
    pub markets: Vec<String>,
    pub selected_market: String,
}

impl Default for AppStore {
    fn default() -> Self {
        AppStore {
            markets: env_utils::get_markets(),
            selected_market: env_utils::get_market()
        }
    }
}