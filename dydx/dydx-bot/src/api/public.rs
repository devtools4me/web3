use actix_web::{get, HttpRequest, Responder, web};
use web::Data;

use crate::api::model::http_response;
use crate::model::app::AppData;

#[get("/markets")]
pub async fn get_markets(req: HttpRequest) -> impl Responder {
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let res = app_data.dydx.get_markets()
        .await
        .map(|x| {
            let keys: Vec<String> = x.keys().into_iter().map(|k| k.clone()).collect();
            keys
        });
    http_response(res)
}