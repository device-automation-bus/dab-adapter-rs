use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartApplicationTelemetryRequest {
    pub appId: String,
    pub duration: u64,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartApplicationTelemetryResponse {
    pub duration: u64,
}
