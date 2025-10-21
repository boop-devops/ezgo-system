use actix_web::web;
use crate::app::controllers::health;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("healthcheck", web::get().to(health::check)),
    );
}
