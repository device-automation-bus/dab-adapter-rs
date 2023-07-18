use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartDeviceTelemetryRequest {
    pub duration: u64,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartDeviceTelemetryResponse {
    pub duration: u64,
}
