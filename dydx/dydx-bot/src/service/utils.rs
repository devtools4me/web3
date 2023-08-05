use chrono::{DateTime, Duration, Utc};

pub fn expiration_unix(minutes: i64) -> i64 {
    let datetime_now: DateTime<Utc> = Utc::now();
    let expiration = datetime_now + Duration::minutes(minutes);
    expiration.timestamp()
}

pub fn format_price(p: f32, tick_size: f32) -> Option<f32> {
    Some(0.0)
}