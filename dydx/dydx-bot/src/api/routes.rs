use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_files::Files;

use crate::model::app::AppData;
use crate::configuration::Settings;
use crate::api::*;

pub const HOME: &str = "/";
pub const TMPL: &str = "/tmpl";
pub const TMPL_NAME: &str = "/tmpl/{name}";
pub const HEALTH: &str = "/health";
pub const ACCOUNT: &str = "/account";
pub const ORDER: &str = "/order";
pub const POSITIONS: &str = "/positions";
pub const CANDLES: &str = "/candles";
pub const CANDLES_MKT_RESOLUTION: &str = "/candles/{market}/{resolution}";

pub fn run_with_config(settings: Settings) -> Result<Server, std::io::Error> {
    let app_data = AppData::new(settings);
    run_with_data(Data::new(app_data))
}

pub fn run_with_data(data: Data<AppData>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .service(
                web::resource(TMPL_NAME)
                    .route(web::get().to(tmpl::render))
            )
            .service(
                web::resource(HEALTH)
                    .route(web::get().to(health::health_check))
            )
            .service(
                web::resource(ACCOUNT)
                    .route(web::get().to(account::get_account))
            )
            .service(
                web::resource(ORDER)
                    .route(web::post().to(trade::create_order))
            )
            .service(
                web::resource(POSITIONS)
                    .route(web::delete().to(trade::close_all_positions))
            )
            .service(
                web::resource(CANDLES_MKT_RESOLUTION)
                    .route(web::get().to(history::get_candles))
            )
            .service(Files::new("/", "./dist/").index_file("index.html"))
    })
        .bind("0.0.0.0:8080")?
        .run();
    Ok(server)
}