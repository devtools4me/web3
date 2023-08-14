use gloo_net::http::Request;
use dydx_api::types::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct DataPoint {
    pub item_name: String,
    pub quantity: f32,
    pub value: f32,
}

pub fn get_data() -> Result<Vec<DataPoint>> {
    let result = vec![
        DataPoint {
            item_name: "item1".to_string(),
            quantity: 105.0,
            value: 0.0,
        }, DataPoint {
            item_name: "item1".to_string(),
            quantity: 10.0,
            value: 30.0,
        },
        DataPoint {
            item_name: "item2".to_string(),
            quantity: 5.0,
            value: -5.0,
        },
        DataPoint {
            item_name: "item2".to_string(),
            quantity: 25.0,
            value: 105.0,
        },
    ];
    wasm_bindgen_futures::spawn_local(async move {
        let fetched_data: Success<Vec<Ohlc>> = Request::get("/candles/BTC-USD/1MIN")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        dbg!("fetched_data={}", fetched_data);
        //result.set(fetched_data);
    });
    Ok(result)
}