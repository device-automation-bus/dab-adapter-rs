use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct StopApplicationTelemetryRequest {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct StopApplicationTelemetryResponse {}
