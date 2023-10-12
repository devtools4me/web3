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

pub fn get_market2() -> String {
    let market = dotenv!("MARKET2");
    String::from(market)
}

pub fn get_markets() -> Vec<String> {
    let market = dotenv!("MARKETS");
    String::from(market)
        .split(",")
        .map(|x| String::from(x))
        .collect()
}

pub fn get_resolution() -> String {
    let resolution = dotenv!("RESOLUTION");
    String::from(resolution)
}

pub fn get_telegram_api_token() -> String {
    let token = dotenv!("TELEGRAM_API_TOKEN");
    String::from(token)
}

pub fn get_telegram_chat_id() -> String {
    let chat_id = dotenv!("TELEGRAM_CHAT_ID");
    String::from(chat_id)
}
