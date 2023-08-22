use actix_web::{get, HttpRequest, Responder, web};
use web::Data;

use crate::api::model::http_response;
use crate::model::app::AppData;

#[get("/averages/sma/{market}/{resolution}")]
pub async fn get_sma(req: HttpRequest) -> impl Responder {
    let market = req.match_info().get("market").unwrap();
    let resolution = req.match_info().get("resolution").unwrap();
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.get_sma(market, resolution).await;
    http_response(res)
}