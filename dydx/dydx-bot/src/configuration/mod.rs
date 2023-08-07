use std::fmt;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub host: String,
    pub client_options: ClientOptions,
}

impl Settings {
    pub fn empty() -> Settings {
        Settings {
            application_port: 0,
            host: "".to_string(),
            client_options: ClientOptions {
                network_id: 0,
                api_key_credentials: ApiKeyCredentials {
                    key: "".to_string(),
                    secret: "".to_string(),
                    passphrase: "".to_string(),
                },
                stark_private_key: "".to_string(),
                eth_address: "".to_string(),
                eth_private_key: "".to_string(),
            },
        }
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Settings(application_port: {}, client_options: {})",
               self.application_port,
               self.client_options)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ClientOptions {
    pub network_id: usize,
    pub api_key_credentials: ApiKeyCredentials,
    pub stark_private_key: String,
    pub eth_address: String,
    pub eth_private_key: String,
}

impl fmt::Display for ClientOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClientOptions(network_id: {}, api_key_credentials: {}, stark_private_key: {}, eth_address: {}, eth_private_key: {})",
               self.network_id,
               self.api_key_credentials,
               self.stark_private_key,
               self.eth_address,
               self.eth_private_key)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiKeyCredentials {
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}

impl fmt::Display for ApiKeyCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiKeyCredentials(key: {}, secret: ****, passphrase: {})",
               self.key,
               self.passphrase)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("conf/configuration"))?;
    settings.try_into()
}