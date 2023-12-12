// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct GetApplicationStateRequest{
// pub appId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct GetApplicationStateResponse{
// pub state: String,
// }

use crate::dab::structs::ErrorResponse;
#[allow(unused_imports)]
use crate::dab::structs::GetApplicationStateRequest;
use crate::dab::structs::GetApplicationStateResponse;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::RdkResponse;
use serde::Deserialize;
use serde_json::json;

pub fn get_app_state (callsign: String) -> Result<String, String> {
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct State {
        callsign: String,
        state: String,
        uri: String,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct GetState {
        state: Vec<State>,
        success: bool,
    }

    let rdkresponse: RdkResponse<GetState> = 
        rdk_request("org.rdk.RDKShell.getState")?;

    for item in rdkresponse.result.state {
        if item.callsign == callsign {
            match item.state.as_str() {
                "suspended" => return Ok("BACKGROUND".to_string()),
                _ => return Ok("FOREGROUND".to_string()),
            }
        }
    }

    Ok("STOPPED".to_string())
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = GetApplicationStateResponse::default();
    // *** Fill in the fields of the struct GetApplicationStateResponse here ***

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
        Ok(_) => (),
    }

    let Dab_Request: GetApplicationStateRequest = IncomingMessage.unwrap();

    if Dab_Request.appId.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'appId' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    ResponseOperator.state = get_app_state(Dab_Request.appId)?;

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
