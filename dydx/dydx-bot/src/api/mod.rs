use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::dev::Server;
use actix_web::web::Data;
use crate::configuration::Settings;
use crate::service::dydx::DydxService;

mod health;
mod trade;

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
            .route("/position/all", web::delete().to(trade::close_all_positions))
            .route("/test", web::get().to(health::health_check))
    })
        .bind("0.0.0.0:8080")?
        .run();
    Ok(server)
}