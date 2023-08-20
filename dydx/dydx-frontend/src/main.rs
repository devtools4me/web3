use log::error;
use yew::prelude::*;

use view::ohlc::OhlcView;
use dydx_api::types::Ohlc;
use utils::api_utils::fetch_single_api_response;

mod data_source;
pub mod mock;
mod client;
mod utils;
mod view;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<OhlcView>::new().render();
}