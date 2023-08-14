use actix_web::HttpResponse;
use log::*;
use dydx_api::types::{Success, Failure};

pub fn http_response<T: serde::ser::Serialize>(result: eyre::Result<T, String>) -> HttpResponse {
    match result {
        Ok(x) => HttpResponse::Ok().json(
            Success {
                value: x
            }
        ),
        Err(err) => {
            warn!("err={}", err);
            HttpResponse::InternalServerError().json(
                Failure {
                    error: err.as_str()
                })
        }
    }
}