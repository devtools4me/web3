use std::sync::Arc;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use web::Data;
use crate::configuration::Settings;
use crate::service::dydx::DydxService;
use crate::service::TradeBot;

pub async fn close_all_positions(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<DydxService>>().unwrap();
    match app_data.close_all_positions() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}