use crate::app::controllers::health;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").route("healthcheck", web::get().to(health::check)));
}
