use std::sync::Arc;
use actix_web::HttpResponse;
use crate::service::TradeBot;

pub async fn close_all_positions(service: Arc<dyn TradeBot>) -> HttpResponse {
    match service.as_ref().close_all_positions() {
        Ok(()) => HttpResponse::Ok(),
        Err(err) => HttpResponse::InternalServerError()
    }.finish()
}