use std::sync::Arc;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use log::*;
use tera::{Context, Tera};

use crate::configuration::Settings;
use crate::service::dydx::DydxService;

mod health;
mod trade;
mod account;
mod history;
mod index;
mod tmpl;

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

struct AppData {
    dydx: DydxService,
    tmpl: Tera,
}

pub fn run(settings: Settings) -> Result<Server, std::io::Error> {
    let tera = Tera::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
    ).unwrap();
    let data = Data::new(
        AppData {
            dydx: DydxService {
                settings
            },
            tmpl: tera,
        }
    );
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index::index))
            )
            .service(
                web::resource("/tmpl/{name}")
                    .route(web::get().to(tmpl::render))
            )
            .service(
                web::resource("/health")
                    .route(web::get().to(health::health_check))
            )
            .route("/health_check", web::get().to(health::health_check))
            .service(
                web::resource("/account")
                    .route(web::get().to(account::get_account))
            )
            .service(
                web::resource("/order")
                    .route(web::post().to(trade::create_order))
            )
            .service(
                web::resource("/positions")
                    .route(web::delete().to(trade::close_all_positions))
            )
            .service(
                web::resource("/candles/{market}/{resolution}")
                    .route(web::get().to(history::get_candles))
            )
    })
        .bind("0.0.0.0:8080")?
        .run();
    Ok(server)
}