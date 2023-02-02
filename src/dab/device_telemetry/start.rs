use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct StartDeviceTelemetryRequest {
    pub frequency: f32,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct StartDeviceTelemetryResponse {
    pub frequency: f32,
}
