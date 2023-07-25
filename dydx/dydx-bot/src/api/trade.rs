use std::sync::Arc;
use actix_web::HttpResponse;
use crate::service::TradeBot;

pub async fn close_all_possitions(service: Arc<dyn TradeBot>) -> HttpResponse {
    service.as_ref().close_all_positions();
    HttpResponse::Ok().finish()
}