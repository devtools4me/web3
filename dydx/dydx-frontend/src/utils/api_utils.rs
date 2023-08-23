use serde::de::DeserializeOwned;
use dydx_common::utils::env_utils;

pub async fn fetch_single_api_response<T: DeserializeOwned>(endpoint: &str) -> reqwest::Result<T> {
    let result = reqwest::get(env_utils::get_api_endpoint() + endpoint)
        .await?
        .json::<T>()
        .await?;
    return Ok(result);
}
