use std::usize;
use dotenvy_macro::dotenv;

pub fn get_api_endpoint() -> String {
    let backend_url = dotenv!("BACKEND_URL");
    String::from(backend_url)
}

pub fn get_window_size() -> u8 {
    let window_size = dotenv!("WINDOW_SIZE");
    String::from(window_size).parse().unwrap()
}

pub fn get_market() -> String {
    let market = dotenv!("MARKET");
    String::from(market)
}

pub fn get_resolution() -> String {
    let resolution = dotenv!("RESOLUTION");
    String::from(resolution)
}
