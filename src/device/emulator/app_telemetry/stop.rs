// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct StopApplicationTelemetryRequest{
// pub appId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct StopApplicationTelemetryResponse {}

#[allow(unused_imports)]
use crate::dab::app_telemetry::stop::StopApplicationTelemetryRequest;
use crate::dab::app_telemetry::stop::StopApplicationTelemetryResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = StopApplicationTelemetryResponse::default();
    // *** Fill in the fields of the struct StopApplicationTelemetryResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
