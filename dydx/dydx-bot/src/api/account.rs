use std::sync::Arc;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use web::Data;
use crate::configuration::Settings;
use crate::service::dydx::DydxService;

pub async fn get_account(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<DydxService>>().unwrap();
    match app_data.get_account().await {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}