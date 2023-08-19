use std::str::FromStr;

use dotenvy_macro::dotenv;
use strum::EnumString;

use dydx_api::types::*;

use crate::client::ReqwestDydxApi;
use crate::mock::{*, MockDydxApi};

#[derive(Clone, Copy, EnumString)]
enum Env {
    Mock,
    Dev,
    Prod,
}

pub async fn get_ohlc_data() -> DydxResult<Vec<Ohlc>> {
    dydx_api_client().get_ohlc_data().await
}

fn dydx_api_client() -> Box<dyn DydxApi + Sync> {
    let env = Env::from_str(dotenv!("ENV")).unwrap();
    match env {
        Env::Mock => Box::new(MockDydxApi {}),
        _ => Box::new(ReqwestDydxApi {})
    }
}