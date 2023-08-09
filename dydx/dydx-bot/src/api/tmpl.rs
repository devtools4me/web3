use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use tera::Context;
use web::Data;

use crate::model::app::AppData;

#[get("/tmpl/{name}")]
pub async fn render(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let app_data = req.app_data::<Data<AppData>>().unwrap();
    let rendered = app_data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
