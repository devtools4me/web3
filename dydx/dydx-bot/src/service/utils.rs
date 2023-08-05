use chrono::{DateTime, Duration, Utc};
use log::debug;

pub fn expiration_unix(minutes: i64) -> i64 {
    let datetime_now: DateTime<Utc> = Utc::now();
    let expiration = datetime_now + Duration::minutes(minutes);
    expiration.timestamp()
}

pub fn format_price(price: f32, tick_size: u32) -> Option<f32> {
    debug!("price={}, tick_size={}", price, tick_size);
    let result = round(price, tick_size);
    debug!("result={}", result);
    Some(result)
}

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10_u32.pow(decimals) as f32;
    (x * y).round() / y
}