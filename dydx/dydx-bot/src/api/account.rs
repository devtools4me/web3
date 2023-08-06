use actix_web::{HttpRequest, Responder, web};
use web::Data;

use crate::api::http_response;
use crate::service::dydx::DydxService;

pub async fn get_account(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<DydxService>>().unwrap();
    let res = app_data.get_account().await;
    http_response(res)
}