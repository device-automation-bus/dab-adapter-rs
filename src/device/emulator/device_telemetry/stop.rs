// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct StopDeviceTelemetryRequest{
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct StopDeviceTelemetryResponse {}

#[allow(unused_imports)]
use crate::dab::device_telemetry::stop::StopDeviceTelemetryRequest;
use crate::dab::device_telemetry::stop::StopDeviceTelemetryResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = StopDeviceTelemetryResponse::default();
    // *** Fill in the fields of the struct StopDeviceTelemetryResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
