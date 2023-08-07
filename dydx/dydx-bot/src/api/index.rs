use actix_web::Responder;

use crate::api::http_response;

pub async fn index() -> impl Responder {
    http_response(Ok("OK"))
}
