use eyre::Result;
use log::*;
use std::{convert::TryFrom, sync::Arc};
use tokio::runtime::Runtime;
use chrono::{DateTime, Duration, Utc};
use crate::constants;
use constants::*;
use dydx_v3_rust::{
    constants::{MAINNET_API_URL, TESTNET_API_URL},
    types::*,
    ClientOptions, DydxClient,
};
use web3::Web3;
use std::time::{SystemTime};

pub fn dydx_create_order_sync() {
    info!("dydx_create_order_sync");
    Runtime::new()
        .unwrap()
        .block_on(dydx_create_order())
        .unwrap();
}

pub async fn dydx_create_order() -> Result<()> {
    let transport = web3::transports::Http::new(HTTP_PROVIDER)?;
    let web3 = web3::Web3::new(transport);
    let api_key = ApiKeyCredentials {
        key: DYDX_API_KEY,
        secret: DYDX_API_SECRET,
        passphrase: DYDX_API_PASSPHRASE,
    };
    let options: ClientOptions = ClientOptions {
        network_id: Some(NETWORK_ID),
        api_timeout: None,
        api_key_credentials: Some(api_key),
        stark_private_key: Some(STARK_PKEY),
        eth_private_key: Some(ETH_PKEY),
    };
    let client = DydxClient::new(HOST, options);
    let private = &client.private.unwrap();
    let response = private.get_account(ETH_ADDRESS).await.unwrap();
    info!("response={:?}", response);
    let position_id = response.account.position_id.as_str();
    info!("position_id={}", position_id);

    let response = client
        .public
        .get_time()
        .await
        .unwrap();
    info!("response={:?}", response);

    let datetime_now: DateTime<Utc> = Utc::now();
    let expiration = datetime_now + Duration::minutes(3);
    let expiration_unix = expiration.timestamp();

    let params: ApiOrderParams = ApiOrderParams {
        position_id: position_id,
        market: DydxMarket::BTC_USD,
        side: OrderSide::BUY,
        type_field: OrderType::MARKET,
        time_in_force: TimeInForce::FOK,
        post_only: false,
        size: "0.001",
        price: "100000",
        limit_fee: "0.015",
        client_id: None,
        cancel_id: None,
        trigger_price: None,
        trailing_percent: None,
        expiration: expiration_unix,
    };
    let order_response = private
        .create_order(params)
        .await
        .unwrap();
    info!("order_response={:?}", order_response);

    Ok(())
}

pub fn dydx_get_time_sync() {
    info!("dydx_get_time");
    Runtime::new()
        .unwrap()
        .block_on(dydx_get_time())
        .unwrap();
}

pub async fn dydx_get_time() -> Result<()> {
    let options: ClientOptions = ClientOptions {
        network_id: None,
        api_timeout: None,
        api_key_credentials: None,
        stark_private_key: None,
        eth_private_key: None,
    };
    let client = DydxClient::new(HOST, options);
    let response = client
        .public
        .get_time()
        .await
        .unwrap();
    info!("response={:?}", response);
    Ok(())
}

pub fn dydx_get_candles_sync() {
    info!("dydx_get_candles");
    Runtime::new()
        .unwrap()
        .block_on(dydx_get_candles())
        .unwrap();
}

pub async fn dydx_get_candles() -> Result<()> {
    let options: ClientOptions = ClientOptions {
        network_id: None,
        api_timeout: None,
        api_key_credentials: None,
        stark_private_key: None,
        eth_private_key: None,
    };
    let client = DydxClient::new(HOST, options);
    let response = client
        .public
        .get_candles(DydxMarket::BTC_USD, Some("1HOUR"), None, None, None)
        .await
        .unwrap();
    info!("response={:?}", response);
    Ok(())
}

pub fn dydx_get_account_sync() {
    info!("dydx_get_account");
    Runtime::new()
        .unwrap()
        .block_on(dydx_get_account())
        .unwrap();
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
        eth_private_key: Some(ETH_PKEY),
    };
    let client = DydxClient::new(HOST, options);
    let private = &client.private.unwrap();
    let response = private.get_account(ETH_ADDRESS).await.unwrap();
    info!("response={:?}", response);
    let position_id = response.account.position_id;
    info!("position_id={}", position_id);
    Ok(())
}

pub fn dydx_get_markets_sync() {
    info!("dydx_get_markets_sync");
    Runtime::new()
        .unwrap()
        .block_on(dydx_get_markets())
        .unwrap();
}

pub async fn dydx_get_markets() -> Result<()> {
    let options: ClientOptions = ClientOptions {
        network_id: None,
        api_timeout: None,
        api_key_credentials: None,
        stark_private_key: None,
        eth_private_key: None,
    };
    let client = DydxClient::new(HOST, options);
    let response = client
        .public
        .get_markets(Some(DydxMarket::BTC_USD))
        .await
        .unwrap();
    info!("response={:?}", response);
    Ok(())
}

pub fn web3_accounts_sync() {
    info!("web3_accounts_sync");
    Runtime::new()
        .unwrap()
        .block_on(web3_accounts())
        .unwrap();
}

pub async fn web3_accounts() -> Result<()> {
    let transport = web3::transports::Http::new(HOST)?;
    let client = Web3::new(transport);
    info!("Calling accounts.");
    let version = client.eth().protocol_version().await?;
    info!("Version: {:?}", version);
    Ok(())
}
