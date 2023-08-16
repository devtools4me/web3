use async_trait::async_trait;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;

use dydx_api::types::*;

pub struct ReqwestDydxApi {}

#[async_trait]
impl DydxApi for ReqwestDydxApi {
    async fn get_ohlc_data(&self) -> Vec<Ohlc> {
        // let fetched_data = reqwest::get("/candles/BTC-USD/1MIN")
        //     .await?
        //     .json::<Vec<Ohlc>>()
        //     .await?;
        // dbg!("fetched_data={}", fetched_data);
        // fetched_data
        todo!()
    }
}