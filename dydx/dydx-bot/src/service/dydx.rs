use crate::service;
use crate::configuration;
use dydx_v3_rust::{
    constants::{MAINNET_API_URL, TESTNET_API_URL},
    types::*,
    ClientOptions, DydxClient,
};

pub struct DydxService<'a> {
    client: DydxClient<'a>
}

impl DydxService {
    pub fn new<'a>(host: &'a str, options: ClientOptions<'a>) -> DydxService<'a> {
        DydxService {
            client: DydxClient::new(host, options)
        }
    }
}

impl service::TradeBot for DydxService {
    fn close_all_positions() -> Result<(), String> {
        todo!()
    }
}