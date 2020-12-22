use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use anyhow::Result;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(debug);
}

#[get("/debug")]
async fn debug(request: web::HttpRequest) -> impl Responder {
    format!(
        "{}{}",
        request.path(),
        request.headers().iter().fold(String::new(), |p, v| {
            format!(
                "{}\n{}: {}",
                p,
                v.0.as_str(),
                String::from_utf8_lossy(v.1.as_bytes())
            )
        })
    )
}
