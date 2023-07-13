use eyre::Result;
use log::*;
use std::{convert::TryFrom, sync::Arc};
use tokio::runtime::Runtime;
//use chrono::{DateTime, Duration, Utc};
use dydx_v3_rust::{
    types::*,
    ClientOptions,
    DydxClient,
    constants::{MAINNET_API_URL, TESTNET_API_URL}
};
use std::time::{Duration, SystemTime};
use crate::constants;
use constants::*;

pub fn dydx_get_account_sync() {
    info!("dydx_get_account");
    Runtime::new().unwrap().block_on(dydx_get_account()).unwrap();
}

pub async fn dydx_get_account() -> Result<()> {
    let transport = web3::transports::Http::new(HTTP_PROVIDER)?;
    let web3 = web3::Web3::new(transport);
    let api_key = ApiKeyCredentials {
        key: DYDX_API_KEY,
        secret: DYDX_API_SECRET,
        passphrase: DYDX_API_PASSPHRASE,
    };
    let options: ClientOptions = ClientOptions {
        network_id: None,
        api_timeout: None,
        api_key_credentials: Some(api_key),
        stark_private_key: Some(STARK_PKEY),
        eth_private_key: Some(ETH_PKEY)
    };
    let client = DydxClient::new(HOST, options);
    let private = &client.private.unwrap();
    let response = private.get_account(ETH_ADDRESS).await.unwrap();
    info!("response={:?}", response);
    Ok(())
}

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