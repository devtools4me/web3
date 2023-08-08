use actix_web::HttpResponse;
use log::*;

#[derive(serde::Serialize)]
pub struct Success<T> {
    pub value: T,
}

#[derive(serde::Serialize)]
pub struct Failure<T> {
    pub error: T,
}

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