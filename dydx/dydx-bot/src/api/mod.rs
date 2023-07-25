use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::dev::Server;
use crate::configuration::Settings;
use crate::configuration;
use crate::service;
use crate::service::dydx::DydxService;
use dydx_v3_rust::ClientOptions;
use dydx_v3_rust::types::ApiKeyCredentials;

mod health;
mod trade;

pub fn run(conf: Settings) -> Result<Server, std::io::Error> {
    let dydx_service = DydxService::new(conf.host.as_str(), client_options(conf.client_options));
    let service = Arc::new(dydx_service);
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health::health_check))
            .route("/position/close_all", web::get().to(trade::close_all_possitions(service.clone())))
    })
        .bind("0.0.0.0:8080")?
        .run();
    Ok(server)
}

fn client_options<'a>(other: configuration::ClientOptions) -> ClientOptions<'a> {
    ClientOptions {
        network_id: Some(other.network_id),
        api_timeout: None,
        api_key_credentials: Some(api_key_credentials(other.api_key_credentials)),
        stark_private_key: Some(other.stark_private_key.as_str()),
        eth_private_key: Some(other.eth_private_key.as_str()),
    }
}

fn api_key_credentials<'a>(other: configuration::ApiKeyCredentials) -> ApiKeyCredentials<'a> {
    ApiKeyCredentials {
        key: other.key.as_str(),
        secret: other.secret.as_str(),
        passphrase: other.passphrase.as_str(),
    }
}