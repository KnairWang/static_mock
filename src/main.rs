mod logger;
mod handler;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::setup();
    log::debug!("logger configured");

    std::panic::set_hook(Box::new(|pi| {
        log::error!("panic: {:?}", pi);
    }));

    HttpServer::new(|| {
        App::new().wrap(Logger::default())
        // .data(model::ConfigContext::default())
        .configure(handler::register)
    })
    .workers(2)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
