use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct HealthCheckRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct HealthCheckResponse {
    pub healthy: bool,
}
