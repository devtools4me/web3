use actix_web::Responder;

use crate::api::http_response;

pub async fn health_check() -> impl Responder {
    http_response(Ok("OK"))
}
