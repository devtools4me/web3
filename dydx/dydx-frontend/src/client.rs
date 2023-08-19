use async_trait::async_trait;
use reqwasm::Error;
use serde::de::DeserializeOwned;
use reqwasm::http::Request;

use dydx_api::types::*;

pub struct ReqwestDydxApi {}

#[async_trait]
impl DydxApi for ReqwestDydxApi {
    async fn get_ohlc_data(&self) -> DydxResult<Vec<Ohlc>> {
        // let resp = Request::get("/candles/BTC-USD/1MIN")
        //     .send()
        //     .await;
        // match resp {
        //     Ok(resp) => {
        //         let json: Result<Vec<Ohlc>, _> = resp.json().await;
        //         match json {
        //             Ok(result) => Ok(result),
        //             Err(e) => Err("error".to_string())
        //         }
        //     },
        //     Err(err) => Err("error".to_string())
        // }
        todo!()
    }
}