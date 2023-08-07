use std::fmt::Debug;

use chrono::{DateTime, Duration, Utc};
use log::{debug, trace, warn};

pub fn eyre<T: Debug>(result: Result<T, Box<dyn std::error::Error>>) -> eyre::Result<T, String> {
    match result {
        Ok(res) => {
            trace!("res={:?}", res);
            Ok(res)
        }
        Err(err) => {
            let msg = err.to_string();
            warn!("failed, err={}", msg);
            Err(msg)
        }
    }
}

pub fn expiration_unix(minutes: i64) -> i64 {
    let datetime_now: DateTime<Utc> = Utc::now();
    let expiration = datetime_now + Duration::minutes(minutes);
    expiration.timestamp()
}

pub fn format_price(price: f32, tick_size: f32) -> Option<f32> {
    debug!("price={}, tick_size={}", price, tick_size);
    let mut mul: f32 = 1.0;
    let mut decimals = 0;
    while tick_size * mul < 1.0 {
        mul = mul * 10.0;
        decimals = decimals + 1;
    }
    let result = round(price, decimals);
    debug!("result={}", result);
    Some(result)
}

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10_u32.pow(decimals) as f32;
    (x * y).round() / y
}