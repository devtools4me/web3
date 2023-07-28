use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::dev::Server;
use actix_web::web::Data;
use crate::configuration;
use configuration::Settings;
use crate::service;
use service::dydx::DydxService;
use dydx_v3_rust::types::ApiKeyCredentials;
use dydx_v3_rust::{
    constants::{MAINNET_API_URL, TESTNET_API_URL},
    types::*,
    ClientOptions, DydxClient,
};

mod health;
mod trade;

pub fn run(conf: &Settings) -> Result<Server, std::io::Error> {
    //let service = DydxService::mk_with_settings(&conf);
    //let data = Data::new(service);
    let data = Data::new(
        DydxService {
            client: DydxClient::new("", ClientOptions {
                network_id: None,
                api_timeout: None,
                api_key_credentials: None,
                stark_private_key: None,
                eth_private_key: None,
            })
        }
    );
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health::health_check))
            .route("/position/all", web::delete().to(trade::close_all_positions))
            .route("/test", web::get().to(health::health_check))
    })
        .bind("0.0.0.0:8080")?
        .run();
    Ok(server)
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