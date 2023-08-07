use actix_web::{HttpRequest, Responder, web};
use web::Data;

use crate::api::http_response;
use crate::model::app::AppData;

pub async fn create_order(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.create_order().await;
    http_response(res)
}

pub async fn close_all_positions(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.close_all_positions().await;
    http_response(res)
}