use actix_web::Responder;

use crate::api::http_response;

pub async fn render() -> impl Responder {
    http_response(Ok("OK"))
}
