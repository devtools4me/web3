use crate::types::{AverageType, IndicatorType};

pub fn candles<'a>(market: &'a str, resolution: &'a str) -> String {
    format!("/candles/{}/{}", market, resolution)
}

pub fn methods<'a>(average_type: AverageType, market: &'a str, resolution: &'a str) -> String {
    format!("/methods/{}/{}/{}", average_type.to_string(), market, resolution)
}

pub fn indicators<'a>(indicator_type: IndicatorType, market: &'a str, resolution: &'a str) -> String {
    format!("/indicators/{}/{}/{}", indicator_type.to_string(), market, resolution)
}