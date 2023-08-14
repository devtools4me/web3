use actix_web::HttpResponse;
use log::*;

pub fn http_response<T: serde::ser::Serialize>(result: eyre::Result<T, String>) -> HttpResponse {
    match result {
        Ok(x) => HttpResponse::Ok().json(
            x
        ),
        Err(err) => {
            warn!("err={}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}