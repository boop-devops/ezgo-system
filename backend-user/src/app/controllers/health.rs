use crate::app::serializers::health::HealthCheckResponse;
use actix_web::HttpResponse;

pub async fn check() -> HttpResponse {
    HttpResponse::Ok().json(HealthCheckResponse::ok())
}
