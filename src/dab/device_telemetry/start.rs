use crate::dab::device_telemetry::DeviceTelemetry;
use crate::dab::ErrorResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[allow(non_snake_case)]
pub fn process(_packet: String, device_telemetry: &mut DeviceTelemetry) -> Result<String, String> {

    let mut ResponseOperator = StartDeviceTelemetryResponse::default();

    let IncomingMessage = serde_json::from_str(&_packet);

    match IncomingMessage {
        Err(err) => {
            let response = ErrorResponse {
                status: 400,
                error: "Error parsing request: ".to_string() + err.to_string().as_str(),
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
        _ => (),
    }

    let Dab_Request: StartDeviceTelemetryRequest = IncomingMessage.unwrap();

    device_telemetry.start(Dab_Request.duration);

    ResponseOperator.duration = Dab_Request.duration;

    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}