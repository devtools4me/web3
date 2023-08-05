use std::sync::Arc;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use web::Data;
use crate::configuration::Settings;
use crate::service::dydx::DydxService;
use log::*;

pub async fn create_order(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<DydxService>>().unwrap();
    match app_data.create_order().await {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}

pub async fn close_all_positions(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<DydxService>>().unwrap();
    match app_data.close_all_positions().await {
        Ok(()) => HttpResponse::Ok(),
        Err(err) => {
            warn!("err={}", err);
            HttpResponse::InternalServerError()
        }
    }
}