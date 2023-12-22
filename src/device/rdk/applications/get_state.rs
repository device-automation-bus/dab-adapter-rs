use crate::dab::structs::DabError;
use crate::dab::structs::GetApplicationStateRequest;
use crate::dab::structs::GetApplicationStateResponse;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::RdkResponse;
use serde::Deserialize;

pub fn get_app_state(callsign: String) -> Result<String, DabError> {
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

    let rdkresponse: RdkResponse<GetState> = rdk_request("org.rdk.RDKShell.getState")?;

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
pub fn process(_dab_request: GetApplicationStateRequest) -> Result<String, DabError> {
    let mut ResponseOperator = GetApplicationStateResponse::default();
    // *** Fill in the fields of the struct GetApplicationStateResponse here ***

    if _dab_request.appId.is_empty() {
        return Err(DabError::Err400(
            "request missing 'appId' parameter".to_string(),
        ));
    }

    ResponseOperator.state = get_app_state(_dab_request.appId)?;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
