use crate::{configuration, service};
use dydx_v3_rust::{
    types::*,
    ClientOptions, DydxClient,
};
use crate::configuration::Settings;
use crate::service::dydx_ops::*;
use crate::service::utils::eyre;

pub struct DydxService {
    pub settings: Settings
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