use std::str::FromStr;

use dydx_v3_rust::{
    ClientOptions,
    DydxClient, types::*,
};
use yata::core::IndicatorResult;

use dydx_api::types::*;
use dydx_common::utils::env_utils;
use dydx_common::utils::vec_utils::*;
use stats::average;
use stats::indicator;

use crate::{configuration, stats};
use crate::configuration::Settings;
use crate::service::dydx_ops::*;
use crate::service::utils::eyre;

pub struct DydxService {
    pub settings: Settings,
}

impl DydxService {
    fn dydx_client(&self) -> DydxClient {
        DydxClient::new(self.settings.host.as_str(), client_options(&self.settings.client_options))
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
}

fn average(average_type: &str, v: Vec<Timeseries>) -> Vec<Timeseries> {
    let t: AverageType = AverageType::from_str(average_type).unwrap();
    let window_size = env_utils::get_window_size();
    match t {
        AverageType::EMA => average::ema(v, window_size),
        AverageType::SMA => average::sma(v, window_size),
        AverageType::HMA => average::hma(v, window_size),
        AverageType::DEMA => average::dema(v, window_size),
        AverageType::DMA => average::dma(v, window_size),
        AverageType::RMA => average::rma(v, window_size),
        AverageType::SWMA => average::swma(v, window_size),
        AverageType::TEMA => average::tema(v, window_size),
        AverageType::TMA => average::tma(v, window_size),
        AverageType::TRIMA => average::trima(v, window_size),
        AverageType::VWMA => average::vwma(v, window_size),
        AverageType::Vidya => average::vidya(v, window_size),
        AverageType::WMA => average::wma(v, window_size),
        AverageType::WSMA => average::wsma(v, window_size),
    }
}

fn indicator(indicator_type: &str, v: Vec<Ohlc>) -> Vec<Indicator> {
    let t: IndicatorType = IndicatorType::from_str(indicator_type).unwrap();
    match t {
        IndicatorType::MACD => indicator::macd(v),
        IndicatorType::RSI => indicator::rsi(v)
    }
}

fn candle_vec_to_owned_ohlc_vec(v: Vec<Candle>) -> Vec<Ohlc> {
    convert(v, |x| Ohlc {
        open: x.open,
        high: x.high,
        low: x.low,
        close: x.close,
        volume: x.base_token_volume,
        timestamp: x.updated_at,
    })
}

fn candle_vec_to_owned_ts_vec(v: Vec<Candle>) -> Vec<Timeseries> {
    convert(v, |x| Timeseries {
        value: x.close,
        timestamp: x.updated_at,
    })
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