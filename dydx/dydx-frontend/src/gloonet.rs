use async_trait::async_trait;
use gloo_net::http::Request;

use dydx_api::types::*;

pub struct GlooNetDydxApi {}

#[async_trait]
impl DydxApi for GlooNetDydxApi {
    async fn get_ohlc_data(&self) -> Vec<Ohlc> {
        // let fetched_data: Vec<Ohlc> = Request::get("/candles/BTC-USD/1MIN")
        //     .send()
        //     .await
        //     .unwrap()
        //     .json()
        //     .await
        //     .unwrap();
        // dbg!("fetched_data={}", fetched_data);
        // fetched_data;
        todo!()
    }
}