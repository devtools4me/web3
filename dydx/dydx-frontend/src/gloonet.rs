use gloo_net::http::Request;

use dydx_api::types::*;

pub struct GlooNetDydxApi {}

impl DydxApi for GlooNetDydxApi {
    fn get_ohlc_data(&self) -> Vec<Ohlc> {
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_data: Vec<Ohlc> = Request::get("/candles/BTC-USD/1MIN")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            dbg!("fetched_data={}", fetched_data);
            //result.set(fetched_data);
        });
        todo!()
    }
}