use actix_web::{get, HttpRequest, Responder, web};
use web::Data;

use crate::api::model::http_response;
use crate::model::app::AppData;

#[get("/methods/{average_type}/{market}/{resolution}")]
pub async fn get_method(req: HttpRequest) -> impl Responder {
    let average_type = req.match_info().get("average_type").unwrap();
    let market = req.match_info().get("market").unwrap();
    let resolution = req.match_info().get("resolution").unwrap();
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.get_average(average_type, market, resolution).await;
    http_response(res)
}

#[get("/indicators/{indicator_type}/{market}/{resolution}")]
pub async fn get_indicator(req: HttpRequest) -> impl Responder {
    let indicator_type = req.match_info().get("indicator_type").unwrap();
    let market = req.match_info().get("market").unwrap();
    let resolution = req.match_info().get("resolution").unwrap();
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.get_indicator(indicator_type, market, resolution).await;
    http_response(res)
}