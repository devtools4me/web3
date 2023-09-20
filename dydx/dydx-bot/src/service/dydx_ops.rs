use std::collections::HashMap;

use async_trait::async_trait;
use dydx_v3_rust::{
    DydxClient,
    types::*,
};
use log::debug;

use service::utils::*;

use crate::service;

#[async_trait]
pub trait DydxOps<'a> {
    async fn get_markets(&self) -> dydx_v3_rust::Result<HashMap<String, MarketData>>;
    async fn get_account(&self, eth_address: &str) -> dydx_v3_rust::Result<AccountObject>;
    async fn place_market_order(&self, eth_address: &str, market: &str, side: &str, size: &str, price: &str) -> dydx_v3_rust::Result<OrderResponseObject>;
    async fn close_all_positions(&self, eth_address: &str) -> dydx_v3_rust::Result<()>;
    async fn get_candles(&self, market: &str, resolution: Option<&str>, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<Vec<Candle>>;
}

#[async_trait]
impl DydxOps<'_> for DydxClient<'_> {
    async fn get_markets(&self) -> dydx_v3_rust::Result<HashMap<String, MarketData>> {
        self.public.get_markets(None)
            .await
            .map(|x| x.markets)
    }

    async fn get_account(&self, eth_address: &str) -> dydx_v3_rust::Result<AccountObject> {
        let private = &self.private.as_ref().unwrap();
        private.get_account(eth_address)
            .await
            .map(|x| x.account)
    }

    async fn place_market_order(&self, eth_address: &str, market: &str, side: &str, size: &str, price: &str) -> dydx_v3_rust::Result<OrderResponseObject> {
        let account = self.get_account(eth_address).await.unwrap();
        let position_id = account.position_id.as_str();
        debug!("position_id={}", position_id);

        let expiration_unix = expiration_unix(3);
        let params: ApiOrderParams = ApiOrderParams {
            position_id,
            market,
            side,
            type_field: OrderType::MARKET,
            time_in_force: TimeInForce::FOK,
            post_only: false,
            size,
            price,
            limit_fee: "0.015",
            client_id: None,
            cancel_id: None,
            trigger_price: None,
            trailing_percent: None,
            expiration: expiration_unix,
        };
        let private = self.private.as_ref().unwrap();
        private
            .create_order(params)
            .await
            .map(|x| x.order)
    }

    async fn close_all_positions(&self, eth_address: &str) -> dydx_v3_rust::Result<()> {
        let markets = self.public.get_markets(None).await.unwrap();
        debug!("markets={:?}", markets);

        let private = &self.private.as_ref().unwrap();
        let response = private.cancel_all_orders(None).await.unwrap();
        debug!("response={:?}", response);

        let positions = private.get_positions(None,
                                              Some("OPEN"),
                                              None,
                                              None)
            .await.unwrap();
        debug!("positions={:?}", positions);

        for p in &positions.positions {
            let market = p.market.as_str();
            let side = match p.side.as_str() {
                "BUY" => "SELL",
                _ => "BUY"
            };
            debug!("side={}", side);

            let price: f32 = p.entry_price.parse().unwrap();
            let accept_price = match side {
                "BUY" => price * 1.7,
                _ => price * 0.3
            };
            debug!("price={}, accept_price={}", price, accept_price);

            let ticked_price = markets.markets.get(market)
                .map(|x| x.tick_size.parse::<f32>().unwrap())
                .and_then(|x| format_price(accept_price, x))
                .unwrap_or(accept_price)
                .to_string();

            let order_response = self.place_market_order(eth_address, market, side, p.size.as_str(), ticked_price.as_str()).await;
            debug!("order_response(OK)={}", order_response.is_ok());
        }

        Ok(())
    }

    async fn get_candles(&self, market: &str, resolution: Option<&str>, from_iso: Option<&str>, to_iso: Option<&str>, limit: Option<&str>) -> dydx_v3_rust::Result<Vec<Candle>> {
        self.public.get_candles(market, resolution, from_iso, to_iso, limit)
            .await
            .map(|x| x.candles)
    }
}