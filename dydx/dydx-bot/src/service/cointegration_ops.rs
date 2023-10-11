use std::error::Error;
use async_trait::async_trait;
use dydx_v3_rust::DydxClient;

use algotrader_common::utils::vec_utils::*;
use algotrader_num::num::with_std_err;
use algotrader_num::types::*;

#[async_trait]
pub trait CointegrationOps<'a> {
    async fn get_cointegration(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<CointResponse>;
    async fn get_spread_zscore(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<SpreadResponse>;
    async fn get_trends(&self, market1: &str, market2: &str, resolution: &str) -> dydx_v3_rust::Result<()>;
    async fn get_candles_close(&self, market: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<Vec<f32>>;
}

#[async_trait]
impl CointegrationOps<'_> for DydxClient<'_> {
    async fn get_cointegration(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<CointResponse> {
        let close1 = self.get_candles_close(market1, resolution, from_iso, to_iso, limit).await?;
        let close2 = self.get_candles_close(market2, resolution, from_iso, to_iso, limit).await?;
        let request = CointRequest {
            series_1: close1,
            series_2: close2,
        };
        let response = algotrader_num::num::get_cointegration(request);
        with_std_err(response)
    }

    async fn get_spread_zscore(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<SpreadResponse> {
        let close1 = self.get_candles_close(market1, resolution, from_iso, to_iso, limit).await?;
        let close2 = self.get_candles_close(market2, resolution, from_iso, to_iso, limit).await?;
        let request = CointRequest {
            series_1: close1.clone(),
            series_2: close2.clone(),
        };
        let coint = with_std_err(algotrader_num::num::get_cointegration(request))?;
        let request = SpreadRequest {
            series_1: close1,
            series_2: close2,
            hedge_ratio: coint.hedge_ratio,
            z_score_window: 21,
        };
        let response = algotrader_num::num::get_spread_z_score(request);
        with_std_err(response)
    }

    async fn get_trends(&self, market1: &str, market2: &str, resolution: &str) -> dydx_v3_rust::Result<()> {
        todo!()
    }

    async fn get_candles_close(&self, market: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<Vec<f32>> {
        self.public.get_candles(
            market,
            Some(resolution),
            from_iso,
            to_iso,
            limit)
            .await
            .map(|x| x.candles)
            .map(|x| reverse(x))
            .map(|x| convert(x, |c| c.close.parse::<f32>().unwrap()))
    }
}