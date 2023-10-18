use actix_web::{get, HttpRequest, Responder, web};
use web::Data;

use crate::api::model::http_response;
use crate::model::app::AppData;

#[get("/cointegration/{market1}/{market2}/{resolution}")]
pub async fn get_cointegration(req: HttpRequest) -> impl Responder {
    let market1 = req.match_info().get("market1").unwrap();
    let market2 = req.match_info().get("market2").unwrap();
    let resolution = req.match_info().get("resolution").unwrap();
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.get_cointegration(market1, market2, resolution).await;
    http_response(res)
}

#[get("/cointegration/spread/{market1}/{market2}/{resolution}")]
pub async fn get_cointegration_spread(req: HttpRequest) -> impl Responder {
    let market1 = req.match_info().get("market1").unwrap();
    let market2 = req.match_info().get("market2").unwrap();
    let resolution = req.match_info().get("resolution").unwrap();
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.get_spread_zscore(market1, market2, resolution).await;
    http_response(res)
}

#[get("/cointegration/trends/{market1}/{market2}/{resolution}")]
pub async fn get_cointegration_trends(req: HttpRequest) -> impl Responder {
    let market1 = req.match_info().get("market1").unwrap();
    let market2 = req.match_info().get("market2").unwrap();
    let resolution = req.match_info().get("resolution").unwrap();
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.get_trends(market1, market2, resolution).await;
    http_response(res)
}