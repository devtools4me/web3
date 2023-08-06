use actix_web::{HttpRequest, Responder, web};
use web::Data;

use crate::api::http_response;
use crate::service::dydx::DydxService;

pub async fn create_order(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<DydxService>>().unwrap();
    let res = app_data.create_order().await;
    http_response(res)
}

pub async fn close_all_positions(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<DydxService>>().unwrap();
    let res = app_data.close_all_positions().await;
    http_response(res)
}