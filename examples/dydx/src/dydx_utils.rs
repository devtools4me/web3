use eyre::Result;
use log::*;
use std::{convert::TryFrom, sync::Arc};
use tokio::runtime::Runtime;
//use chrono::{DateTime, Duration, Utc};
use dydx_v3_rust::{types::*, ClientOptions, DydxClient};
use std::time::{Duration, SystemTime};

pub fn dydx_get_markets_sync() {
    info!("dydx_get_markets_sync");
    Runtime::new().unwrap().block_on(dydx_get_markets()).unwrap();
}

pub async fn dydx_get_markets() -> Result<()> {
    let options: ClientOptions = ClientOptions {
        network_id: None,
        api_timeout: None,
        api_key_credentials: None,
        stark_private_key: None,
        eth_private_key: None,
    };
    let client = DydxClient::new("https://api.dydx.exchange", options);
    let response = client
        .public
        .get_markets(Some(DydxMarket::BTC_USD))
        .await
        .unwrap();
    info!("response={:?}", response);
    Ok(())
}