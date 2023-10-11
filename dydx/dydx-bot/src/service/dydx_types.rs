use dydx_v3_rust::{
    types::*,
};

use algotrader_api::types::*;
use algotrader_common::utils::vec_utils::*;
use algotrader_num::types::SpreadResponse;

pub fn candle_vec_to_owned_ohlc_vec(v: Vec<Candle>) -> Vec<Ohlc> {
    convert(v, |x| Ohlc {
        open: x.open,
        high: x.high,
        low: x.low,
        close: x.close,
        volume: x.base_token_volume,
        timestamp: x.updated_at,
    })
}

pub fn candle_vec_to_owned_ts_vec(v: Vec<Candle>) -> Vec<Timeseries> {
    convert(v, |x| Timeseries {
        value: x.close,
        timestamp: x.updated_at,
    })
}

pub fn sread_vec_to_zscore_ts_vec(other: SpreadResponse) -> Vec<Timeseries> {
    convert(other.z_score, |x| Timeseries {
        value: x,
        timestamp: String::from(""),
    })
}

pub fn sread_vec_to_spread_ts_vec(other: SpreadResponse) -> Vec<Timeseries> {
    convert(other.spread, |x| Timeseries {
        value: x,
        timestamp: String::from(""),
    })
}