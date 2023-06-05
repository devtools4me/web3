use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::dev::Server;

mod health;

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health::health_check))
    })
        .bind("0.0.0.0:8080")?
        .run();
    Ok(server)
}