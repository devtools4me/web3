use actix_web::{get, Responder};

use crate::api::model::http_response;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    http_response(Ok("OK"))
}
