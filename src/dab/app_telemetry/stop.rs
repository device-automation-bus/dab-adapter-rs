use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopApplicationTelemetryRequest {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopApplicationTelemetryResponse {}
