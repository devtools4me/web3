use dotenvy_macro::dotenv;

pub fn get_api_endpoint() -> String {
    let backend_url = dotenv!("BACKEND_URL");
    String::from(backend_url)
}
