use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: bool,
    pub message: String,
}

impl HealthCheckResponse {
    pub fn ok() -> Self {
        Self {
            status: true,
            message: "OK".to_string(),
        }
    }
}
