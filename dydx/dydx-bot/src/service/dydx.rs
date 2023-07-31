use crate::{configuration, service};
use dydx_v3_rust::{
    constants::{MAINNET_API_URL, TESTNET_API_URL},
    types::*,
    ClientOptions, DydxClient,
};
use chrono::{DateTime, Duration, Utc};
use log::debug;
use crate::configuration::Settings;

pub struct DydxService {
    pub settings: Settings
}

impl DydxService {
    fn dydx_client(&self) -> DydxClient {
        DydxClient::new(self.settings.host.as_str(), client_options(&self.settings.client_options))
    }

    pub async fn get_account(&self) -> eyre::Result<(), String> {
        let client = self.dydx_client();
        let private = &client.private.unwrap();
        let eth_address = self.settings.client_options.eth_address.as_str();
        let response = private.get_account(eth_address).await.unwrap();
        debug!("response={:?}", response);
        Ok(())
    }

    pub async fn create_order(&self) -> eyre::Result<(), String> {
        let client = self.dydx_client();
        let private = &client.private.unwrap();
        let eth_address = self.settings.client_options.eth_address.as_str();
        let response = private.get_account(eth_address).await.unwrap();
        debug!("response={:?}", response);

        let position_id = response.account.position_id.as_str();
        debug!("position_id={}", position_id);

        let response = client
            .public
            .get_time()
            .await
            .unwrap();
        debug!("response={:?}", response);

        let expiration_unix = expiration_unix(3);
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
        debug!("order_response={:?}", order_response);
        Ok(())
    }

    pub async fn close_all_positions(&self) -> Result<(), String> {
        let client = self.dydx_client();
        Ok(())
    }
}

fn expiration_unix(minutes: i64) -> i64 {
    let datetime_now: DateTime<Utc> = Utc::now();
    let expiration = datetime_now + Duration::minutes(minutes);
    expiration.timestamp()
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