use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct StartApplicationTelemetryRequest {
    pub appId: String,
    pub frequency: f32,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct StartApplicationTelemetryResponse {
    pub frequency: f32,
}
