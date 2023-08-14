use actix_web::{App, HttpServer, HttpResponse, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_files::Files;

use crate::model::app::AppData;
use crate::configuration::Settings;
use crate::api::*;

pub fn run_with_config(settings: Settings) -> Result<Server, std::io::Error> {
    let app_data = AppData::new(settings);
    run_with_data(Data::new(app_data))
}

pub fn run_with_data(data: Data<AppData>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .service(tmpl::render)
            .service(health::health_check)
            .service(account::get_account)
            .service(trade::create_order)
            .service(trade::close_all_positions)
            .service(history::get_candles)
            .service(Files::new("/", "./dist/").index_file("index.html"))
    })
        .bind("0.0.0.0:8000")?
        .run();
    Ok(server)
}