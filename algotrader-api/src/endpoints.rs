use crate::types::{AverageType, IndicatorType};

pub fn candles<'a>(market: &'a str, resolution: &'a str) -> String {
    format!("/candles/{}/{}", market, resolution)
}

pub fn methods<'a>(average_type: &'a AverageType, market: &'a str, resolution: &'a str) -> String {
    format!("/methods/{}/{}/{}", average_type.to_string(), market, resolution)
}

pub fn indicators<'a>(indicator_type: &'a IndicatorType, market: &'a str, resolution: &'a str) -> String {
    format!("/indicators/{}/{}/{}", indicator_type.to_string(), market, resolution)
}

pub fn cointegration<'a>(market1: &'a str, market2: &'a str, resolution: &'a str) -> String {
    format!("/cointegration/{}/{}/{}", market1, market2, resolution)
}

pub fn cointegration_zscore<'a>(market1: &'a str, market2: &'a str, resolution: &'a str) -> String {
    format!("/cointegration/zscore/{}/{}/{}", market1, market2, resolution)
}

pub fn cointegration_spread<'a>(market1: &'a str, market2: &'a str, resolution: &'a str) -> String {
    format!("/cointegration/spread/{}/{}/{}", market1, market2, resolution)
}

pub fn cointegration_trends<'a>(market1: &'a str, market2: &'a str, resolution: &'a str) -> String {
    format!("/cointegration/trends/{}/{}/{}", market1, market2, resolution)
}

pub fn markets() -> String {
    format!("/markets")
}

