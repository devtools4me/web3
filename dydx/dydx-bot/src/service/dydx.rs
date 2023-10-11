use std::collections::HashMap;
use std::str::FromStr;

use dydx_v3_rust::{
    ClientOptions,
    DydxClient, types::*,
};

use algotrader_api::types::*;
use algotrader_common::utils::env_utils;
use algotrader_common::utils::vec_utils::*;
use algotrader_num::types::CointResponse;
use algotrader_ta::indicators;
use algotrader_ta::methods;

use crate::configuration;
use crate::configuration::Settings;
use crate::service::cointegration_ops::*;
use crate::service::dydx_ops::*;
use crate::service::dydx_types::*;
use crate::service::utils::eyre;

pub struct DydxService {
    pub settings: Settings,
}

impl DydxService {
    fn dydx_client(&self) -> DydxClient {
        DydxClient::new(self.settings.host.as_str(), client_options(&self.settings.client_options))
    }

    pub async fn get_markets(&self) -> eyre::Result<HashMap<String, MarketData>, String> {
        let client = self.dydx_client();
        let result = client.get_markets()
            .await;
        eyre(result)
    }

    pub async fn get_account(&self) -> eyre::Result<AccountObject, String> {
        let client = self.dydx_client();
        let result = client.get_account(self.settings.client_options.eth_address.as_str())
            .await;
        eyre(result)
    }

    pub async fn create_order(&self) -> eyre::Result<OrderResponseObject, String> {
        let client: DydxClient = self.dydx_client();
        let result = client.place_market_order(
            self.settings.client_options.eth_address.as_str(),
            DydxMarket::BTC_USD,
            OrderSide::BUY,
            "0.001",
            "100000")
            .await;
        eyre(result)
    }

    pub async fn close_all_positions(&self) -> Result<(), String> {
        let client = self.dydx_client();
        let result = client.close_all_positions(self.settings.client_options.eth_address.as_str())
            .await;
        eyre(result)
    }

    pub async fn get_candles(&self, market: &str, resolution: &str) -> eyre::Result<Vec<Ohlc>, String> {
        let client = self.dydx_client();
        let result = client.get_candles(
            market,
            Some(resolution),
            None,
            None,
            Some("100"))
            .await
            .map(|x| reverse(x))
            .map(|x| candle_vec_to_owned_ohlc_vec(x));
        eyre(result)
    }

    pub async fn get_average(&self, average_type: &str, market: &str, resolution: &str) -> eyre::Result<Vec<Timeseries>, String> {
        let client = self.dydx_client();
        let result = client.get_candles(
            market,
            Some(resolution),
            None,
            None,
            Some("100"))
            .await
            .map(|x| reverse(x))
            .map(|x| candle_vec_to_owned_ts_vec(x))
            .map(|x| average(average_type, x));
        eyre(result)
    }

    pub async fn get_indicator(&self, indicator_type: &str, market: &str, resolution: &str) -> eyre::Result<Vec<Indicator>, String> {
        let client = self.dydx_client();
        let result = client.get_candles(
            market,
            Some(resolution),
            None,
            None,
            Some("100"))
            .await
            .map(|x| reverse(x))
            .map(|x| candle_vec_to_owned_ohlc_vec(x))
            .map(|x| indicator(indicator_type, x));
        eyre(result)
    }

    pub async fn get_cointegration(&self, market1: &str, market2: &str, resolution: &str) -> eyre::Result<CointResponse, String> {
        let client = self.dydx_client();
        let result = client.get_cointegration(market1, market2, resolution, None, None, None)
            .await;
        eyre(result)
    }

    pub async fn get_zscore(&self, market1: &str, market2: &str, resolution: &str) -> eyre::Result<Vec<Timeseries>, String> {
        let client = self.dydx_client();
        let result = client.get_spread(market1, market2, resolution)
            .await
            .map(|x| sread_vec_to_zscore_ts_vec(x));
        eyre(result)
    }

    pub async fn get_spread(&self, market1: &str, market2: &str, resolution: &str) -> eyre::Result<Vec<Timeseries>, String> {
        let client = self.dydx_client();
        let result = client.get_spread(market1, market2, resolution)
            .await
            .map(|x| sread_vec_to_spread_ts_vec(x));
        eyre(result)
    }

    pub async fn get_trends(&self, market1: &str, market2: &str, resolution: &str) -> eyre::Result<Vec<Timeseries>, String> {
        let client = self.dydx_client();
        let result = client.get_spread(market1, market2, resolution)
            .await
            .map(|x| sread_vec_to_spread_ts_vec(x));
        eyre(result)
    }
}

fn average(average_type: &str, v: Vec<Timeseries>) -> Vec<Timeseries> {
    let t: AverageType = AverageType::from_str(average_type).unwrap();
    let window_size = env_utils::get_window_size();
    match t {
        AverageType::EMA => methods::ema(v, window_size),
        AverageType::SMA => methods::sma(v, window_size),
        AverageType::HMA => methods::hma(v, window_size),
        AverageType::DEMA => methods::dema(v, window_size),
        AverageType::DMA => methods::dma(v, window_size),
        AverageType::Momentum => methods::momentum(v, window_size),
        AverageType::RMA => methods::rma(v, window_size),
        AverageType::SWMA => methods::swma(v, window_size),
        AverageType::TEMA => methods::tema(v, window_size),
        AverageType::TMA => methods::tma(v, window_size),
        AverageType::TRIMA => methods::trima(v, window_size),
        AverageType::VWMA => methods::vwma(v, window_size),
        AverageType::Vidya => methods::vidya(v, window_size),
        AverageType::WMA => methods::wma(v, window_size),
        AverageType::WSMA => methods::wsma(v, window_size),
    }
}

fn indicator(indicator_type: &str, v: Vec<Ohlc>) -> Vec<Indicator> {
    let t: IndicatorType = IndicatorType::from_str(indicator_type).unwrap();
    match t {
        IndicatorType::MACD => indicators::macd(v),
        IndicatorType::RSI => indicators::rsi(v),
        IndicatorType::RunTogether => indicators::run_together(v),
        IndicatorType::SellVolatility => indicators::sell_volatility(v),
    }
}

fn client_options(other: &configuration::ClientOptions) -> ClientOptions {
    ClientOptions {
        network_id: Some(other.network_id),
        api_timeout: None,
        api_key_credentials: Some(api_key_credentials(&other.api_key_credentials)),
        stark_private_key: Some(other.stark_private_key.as_str()),
        eth_private_key: Some(other.eth_private_key.as_str()),
    }
}

fn api_key_credentials(other: &configuration::ApiKeyCredentials) -> ApiKeyCredentials {
    ApiKeyCredentials {
        key: other.key.as_str(),
        secret: other.secret.as_str(),
        passphrase: other.passphrase.as_str(),
    }
}