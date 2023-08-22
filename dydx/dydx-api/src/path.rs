
pub fn candles<'a>(market: &'a str, resolution: &'a str) -> String {
    format!("/candles/{}/{}", market, resolution)
}

pub mod averages {
    pub fn sma<'a>(market: &'a str, resolution: &'a str) -> String {
        format!("/averages/sma/{}/{}", market, resolution)
    }
}

pub mod methods {
    pub fn momentum<'a>(market: &'a str, resolution: &'a str) -> String {
        format!("/averages/sma/{}/{}", market, resolution)
    }
}