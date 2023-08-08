use actix_web::{HttpRequest, Responder, web};
use web::Data;

use crate::api::model::http_response;
use crate::model::app::AppData;

pub async fn get_account(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.get_account().await;
    http_response(res)
}