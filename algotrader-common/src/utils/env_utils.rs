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
    let markets = dotenv!("MARKETS");
    String::from(markets)
        .split(",")
        .map(|x| String::from(x))
        .collect()
}

pub fn get_resolution() -> String {
    let resolution = dotenv!("RESOLUTION");
    String::from(resolution)
}

pub fn get_method() -> String {
    let market = dotenv!("METHOD");
    String::from(market)
}

pub fn get_methods() -> Vec<String> {
    let methods = dotenv!("METHODS");
    String::from(methods)
        .split(",")
        .map(|x| String::from(x))
        .collect()
}

pub fn get_indicator() -> String {
    let market = dotenv!("INDICATOR");
    String::from(market)
}

pub fn get_indicators() -> Vec<String> {
    let methods = dotenv!("INDICATORS");
    String::from(methods)
        .split(",")
        .map(|x| String::from(x))
        .collect()
}

pub fn get_telegram_api_token() -> String {
    let token = dotenv!("TELEGRAM_API_TOKEN");
    String::from(token)
}

pub fn get_telegram_chat_id() -> String {
    let chat_id = dotenv!("TELEGRAM_CHAT_ID");
    String::from(chat_id)
}

pub fn get_eth_url() -> String {
    let res = dotenv!("ETH_URL");
    String::from(res)
}

pub fn get_weth_addr() -> String {
    let res = dotenv!("WETH_ADDR");
    String::from(res)
}

pub fn get_usdc_addr() -> String {
    let res = dotenv!("USDC_ADDR");
    String::from(res)
}

pub fn get_wallet_addr() -> String {
    let res = dotenv!("WALLET_ADDR");
    String::from(res)
}

pub fn get_wallet_priv_key() -> String {
    let res = dotenv!("WALLET_PRIV_KEY");
    String::from(res)
}

pub fn get_usdc_whale() -> String {
    let res = dotenv!("USDC_WHALE");
    String::from(res)
}

pub fn get_addr_router() -> String {
    let res = dotenv!("ADDR_ROUTER");
    String::from(res)
}
