// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct StartDeviceTelemetryRequest{
// pub frequency: f32,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct StartDeviceTelemetryResponse{
// pub frequency: f32,
// }

#[allow(unused_imports)]
use crate::dab::device_telemetry::start::StartDeviceTelemetryRequest;
use crate::dab::device_telemetry::start::StartDeviceTelemetryResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = StartDeviceTelemetryResponse::default();
    // *** Fill in the fields of the struct StartDeviceTelemetryResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
