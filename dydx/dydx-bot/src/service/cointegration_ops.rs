use async_trait::async_trait;
use dydx_v3_rust::DydxClient;

use algotrader_common::utils::vec_utils::*;
use algotrader_num::types::*;

#[async_trait]
pub trait CointegrationOps<'a> {
    async fn get_cointegration(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<CointResponse>;
    async fn get_spread(&self, market1: &str, market2: &str, resolution: &str) -> dydx_v3_rust::Result<SpreadResponse>;
    async fn get_trends(&self, market1: &str, market2: &str, resolution: &str) -> dydx_v3_rust::Result<()>;
}

#[async_trait]
impl CointegrationOps<'_> for DydxClient<'_> {
    async fn get_cointegration(&self, market1: &str, market2: &str, resolution: &str, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<CointResponse> {
        let close1 = self.public.get_candles(
            market1,
            Some(resolution),
            from_iso,
            to_iso,
            limit)
            .await
            .map(|x| x.candles)
            .map(|x| reverse(x))
            .map(|x| convert(x, |c| c.close.parse::<f32>().unwrap()))?;
        let close2 = self.public.get_candles(
            market2,
            Some(resolution),
            from_iso,
            to_iso,
            limit)
            .await
            .map(|x| x.candles)
            .map(|x| reverse(x))
            .map(|x| convert(x, |c| c.close.parse::<f32>().unwrap()))?;
        let request = CointRequest {
            series_1: close1,
            series_2: close2,
        };
        let response = algotrader_num::num::get_cointegration(request);
        let result = match response {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string().as_str().into())
        };
        result
    }

    async fn get_spread(&self, market1: &str, market2: &str, resolution: &str) -> dydx_v3_rust::Result<SpreadResponse> {
        todo!()
    }

    async fn get_trends(&self, market1: &str, market2: &str, resolution: &str) -> dydx_v3_rust::Result<()> {
        todo!()
    }
}