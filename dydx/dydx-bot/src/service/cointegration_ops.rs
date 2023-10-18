use async_trait::async_trait;
use dydx_v3_rust::DydxClient;

use algotrader_api::types::{CointegrationData, SpreadZScoreData, TimeseriesF32, TrendsData};
use algotrader_common::utils::vec_utils::*;
use algotrader_num::num::with_std_err;
use algotrader_num::types::*;

#[async_trait]
pub trait CointegrationOps<'a> {
    async fn get_cointegration(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<CointResponse>;
    async fn get_spread_zscore(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<SpreadZScoreData>;
    async fn get_trends(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<TrendsData>;
    async fn get_candles_close(&self, market: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<Vec<TimeseriesF32>>;
}

#[async_trait]
impl CointegrationOps<'_> for DydxClient<'_> {
    async fn get_cointegration(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<CointResponse> {
        let timeseries1 = self.get_candles_close(market1, resolution, from_iso, to_iso, limit).await?;
        let close1 = convert(timeseries1, |x: TimeseriesF32| x.value);
        let timeseries2 = self.get_candles_close(market2, resolution, from_iso, to_iso, limit).await?;
        let close2 = convert(timeseries2, |x: TimeseriesF32| x.value);
        let request = CointRequest {
            series_1: close1,
            series_2: close2,
        };
        let response = algotrader_num::num::get_cointegration(request);
        with_std_err(response)
    }

    async fn get_spread_zscore(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<SpreadZScoreData> {
        let timeseries1 = self.get_candles_close(market1, resolution, from_iso, to_iso, limit).await?;
        let close1 = convert(timeseries1.clone(), |x: TimeseriesF32| x.value);
        let timestamp1 = convert(timeseries1, |x: TimeseriesF32| x.timestamp);
        let timeseries2 = self.get_candles_close(market2, resolution, from_iso, to_iso, limit).await?;
        let close2 = convert(timeseries2, |x: TimeseriesF32| x.value);
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
        let response = algotrader_num::num::get_spread_z_score(request)
            .map(|x| spread_zscore(&x, &timestamp1));
        with_std_err(response)
    }

    async fn get_trends(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<TrendsData> {
        let cointegration = self.get_cointegration(market1, market2, resolution, from_iso, to_iso, limit).await?;
        let spread = self.get_spread_zscore(market1, market2, resolution, from_iso, to_iso, limit).await?;
        Ok(
            TrendsData {
                cointegration_data: cointegration_data(&cointegration),
                spread_zscore_data: spread
            })
    }

    async fn get_candles_close(&self, market: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<Vec<TimeseriesF32>> {
        self.public.get_candles(
            market,
            Some(resolution),
            from_iso,
            to_iso,
            limit)
            .await
            .map(|x| x.candles)
            .map(|x| reverse(x))
            .map(|x| {
                convert(x, |c| {
                    TimeseriesF32 {
                        value: c.close.parse::<f32>().unwrap(),
                        timestamp: c.updated_at.clone(),
                    }
                })
            })
    }
}

fn cointegration_data(other: &CointResponse) -> CointegrationData {
    CointegrationData {
        p_value: other.p_value,
        coint_t: other.coint_t,
        c_value: other.c_value,
        hedge_ratio: other.hedge_ratio,
        zero_crossings: other.zero_crossings,
    }
}

fn spread_zscore(other: &SpreadResponse, timestamp: &Vec<String>) -> SpreadZScoreData {
    SpreadZScoreData {
        spread: other.spread.clone(),
        z_score: other.z_score.clone(),
        timestamp: timestamp.clone(),
    }
}