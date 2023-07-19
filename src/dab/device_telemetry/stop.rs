use crate::dab::device_telemetry::DeviceTelemetry;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopDeviceTelemetryRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopDeviceTelemetryResponse {}

#[allow(non_snake_case)]
pub fn process(_packet: String, device_telemetry: &mut DeviceTelemetry) -> Result<String, String> {
    let ResponseOperator = StopDeviceTelemetryResponse::default();

    device_telemetry.stop();

    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}