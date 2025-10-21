use actix_web::HttpResponse;
use crate::app::serializers::health::HealthCheckResponse;

pub async fn check() -> HttpResponse {
    HttpResponse::Ok().json(HealthCheckResponse::ok())
}
