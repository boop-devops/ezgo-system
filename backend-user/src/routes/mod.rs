use actix_web::web;

pub mod health;

pub fn configure(cfg: &mut web::ServiceConfig) {
    health::routes(cfg);
}
