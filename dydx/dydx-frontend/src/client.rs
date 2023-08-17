use async_trait::async_trait;
use serde::de::DeserializeOwned;
use reqwasm::http::Request;

use dydx_api::types::*;

pub struct ReqwestDydxApi {}

#[async_trait]
impl DydxApi for ReqwestDydxApi {
    async fn get_ohlc_data(&self) -> Result<Vec<Ohlc>> {
        // let fetched_data = Request::get("/candles/BTC-USD/1MIN")
        //     .send()
        //     .await?;
        // let x: Vec<Ohlc> = fetched_data.json().await.unwrap();
        // Ok(x);
        todo!()
    }
}