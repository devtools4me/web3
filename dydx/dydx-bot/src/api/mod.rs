use std::sync::Arc;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use log::*;

use crate::configuration::Settings;
use crate::service::dydx::DydxService;

mod health;
mod trade;
mod account;
mod history;

#[derive(serde::Serialize)]
pub struct Success<T> {
    value: T,
}

#[derive(serde::Serialize)]
pub struct Error<T> {
    error: T,
}

pub fn http_response<T: serde::ser::Serialize>(result: eyre::Result<T, String>) -> HttpResponse {
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(err) => {
            warn!("err={}", err);
            HttpResponse::InternalServerError().json(
                Error {
                    error: err.as_str()
                })
        }
    }
}

pub fn run(settings: Settings) -> Result<Server, std::io::Error> {
    let data = Data::new(
        DydxService {
            settings
        }
    );
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health::health_check))
            .route("/account", web::get().to(account::get_account))
            .route("/order", web::post().to(trade::create_order))
            .route("/position/all", web::delete().to(trade::close_all_positions))
            .route("/history/candles", web::get().to(history::get_candles))
            .route("/test", web::get().to(health::health_check))
    })
        .bind("0.0.0.0:8080")?
        .run();
    Ok(server)
}