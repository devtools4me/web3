use crate::{configuration, service};
use service::TradeBot;
use dydx_v3_rust::{
    constants::{MAINNET_API_URL, TESTNET_API_URL},
    types::*,
    ClientOptions, DydxClient,
};

pub struct DydxService<'a> {
    pub client: DydxClient<'a>
}

impl DydxService<'_> {
    pub fn new<'a>(host: &'a str, options: ClientOptions<'a>) -> DydxService<'a> {
        DydxService {
            client: DydxClient::new(host, options)
        }
    }

    pub fn mk_with_settings<'a>(settings: &configuration::Settings) -> DydxService {
        let api_key = ApiKeyCredentials {
            key: settings.client_options.api_key_credentials.key.as_str(),
            secret: settings.client_options.api_key_credentials.secret.as_str(),
            passphrase: settings.client_options.api_key_credentials.passphrase.as_str(),
        };
        let options: ClientOptions = ClientOptions {
            network_id: Some(settings.client_options.network_id),
            api_timeout: None,
            api_key_credentials: Some(api_key),
            stark_private_key: Some(settings.client_options.stark_private_key.as_str()),
            eth_private_key: Some(settings.client_options.eth_private_key.as_str()),
        };
        DydxService {
            client: DydxClient::new(settings.host.as_str(), options)
        }
    }
}

impl TradeBot for DydxService<'_> {
    fn close_all_positions(&self) -> Result<(), String> {
        Ok(())
    }
}