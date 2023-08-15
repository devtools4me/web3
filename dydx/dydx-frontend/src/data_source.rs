use dydx_api::types::*;

use crate::mock::{*, MockDydxApi};
use crate::gloonet::GlooNetDydxApi;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy)]
enum Env {
    Mock,
    Dev,
    Prod
}

static ENV: Env = Env::Mock;
//static ENV: Env = Env::Dev;

pub fn get_data() -> Result<Vec<Ohlc>> {
    let response = dydx_api_client(ENV).get_ohlc_data();
    Ok(response)
}

fn dydx_api_client(env: Env) -> Box<dyn DydxApi + Sync> {
    match env {
        Env::Mock => Box::new(MockDydxApi {}),
        _ => Box::new(GlooNetDydxApi {})
    }
}