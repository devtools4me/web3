use crate::types::AverageType;

pub fn candles<'a>(market: &'a str, resolution: &'a str) -> String {
    format!("/candles/{}/{}", market, resolution)
}

pub fn averages<'a>(average_type: AverageType, market: &'a str, resolution: &'a str) -> String {
    format!("/averages/{}/{}/{}", average_type.to_string(), market, resolution)
}

pub mod methods {
    pub fn momentum<'a>(market: &'a str, resolution: &'a str) -> String {
        format!("/averages/sma/{}/{}", market, resolution)
    }
}