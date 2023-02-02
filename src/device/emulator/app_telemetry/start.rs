// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct StartApplicationTelemetryRequest{
// pub appId: String,
// pub frequency: f32,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct StartApplicationTelemetryResponse{
// pub frequency: f32,
// }

#[allow(unused_imports)]
use crate::dab::app_telemetry::start::StartApplicationTelemetryRequest;
use crate::dab::app_telemetry::start::StartApplicationTelemetryResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = StartApplicationTelemetryResponse::default();
    // *** Fill in the fields of the struct StartApplicationTelemetryResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
