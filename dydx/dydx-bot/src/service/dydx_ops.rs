use crate::{configuration, service};
use dydx_v3_rust::{
    types::*,
    ClientOptions, DydxClient,
};
use dydx_v3_rust::modules::private::Private;
use log::debug;
use async_trait::async_trait;
use configuration::Settings;
use service::utils::*;

#[async_trait]
pub trait DydxOps<'a> {
    async fn get_account(&self, eth_address: &str) -> eyre::Result<(), String>;
    async fn place_market_order(&self, eth_address: &str, market: &str, side: &str, size: &str, price: &str) -> eyre::Result<(), String>;
    async fn close_all_positions(&self, eth_address: &str) -> eyre::Result<(), String>;
}

#[async_trait]
impl DydxOps<'_> for DydxClient<'_> {
    async fn get_account(&self, eth_address: &str) -> eyre::Result<(), String> {
        let private = &self.private.as_ref().unwrap();
        let response = private.get_account(eth_address).await.unwrap();
        debug!("response={:?}", response);
        Ok(())
    }

    async fn place_market_order(&self, eth_address: &str, market: &str, side: &str, size: &str, price: &str) -> eyre::Result<(), String>{
        let private = self.private.as_ref().unwrap();
        let response = private.get_account(eth_address).await.unwrap();
        debug!("response={:?}", response);

        let position_id = response.account.position_id.as_str();
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
        let order_response = private
            .create_order(params)
            .await
            .unwrap();
        debug!("order_response={:?}", order_response);
        Ok(())
    }

    async fn close_all_positions(&self, eth_address: &str) -> Result<(), String> {
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

            //let result = private.place_market_order(eth_address, market, side, p.size.as_str(), ticked_price.as_str()).await.unwrap()
        }

        Ok(())
    }
}