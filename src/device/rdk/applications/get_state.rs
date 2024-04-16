use crate::dab::structs::DabError;
use crate::dab::structs::GetApplicationStateRequest;
use crate::dab::structs::GetApplicationStateResponse;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::RdkResponse;
use crate::hw_specific::applications::launch::get_visibility;
use serde::Deserialize;

/**
 * DAB App State Mapping:
 *   STOPPED: Application instance is not running.
 *   FOREGROUND: Application is visible active & focused(accepting inputs).
 *   BACKGROUND: Application instance is running but not visible & focused.
*/
#[derive(Debug)]
pub enum DABAppState {
    Stopped,
    Background,
    Foreground,
}

impl DABAppState {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DABAppState::Stopped => "STOPPED",
            DABAppState::Background => "BACKGROUND",
            DABAppState::Foreground => "FOREGROUND",
        }
    }
}

/*
 * Returns the state of the application mapped to DAB application state.
 * @param callsign: String
 * @return String: DAB application state
*/
pub fn get_dab_app_state(callsign: String) -> Result<String, DabError> {
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
                "suspended" => return Ok(DABAppState::Background.as_str().to_string()),
                "activated" | "resumed" => {
                    // Launch request mandates that application should be focused and visible.
                    let visibility = get_visibility(callsign)?;
                    let app_state = if visibility {
                        DABAppState::Foreground
                    } else {
                        DABAppState::Background
                    };
                    return Ok(app_state.as_str().to_string());
                },
                _ => {
                    println!("Implement verification of: {} App state: {}",
                        callsign.clone(), item.state.as_str());
                    return Err(DabError::Err500(
                        format!("RDKShell.getState; {} is in invalid state {}.",
                            callsign.clone(), item.state.as_str()).to_string(),
                    ));
                }
            }
        }
    }

    Ok(DABAppState::Stopped.as_str().to_string())
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

    ResponseOperator.state = get_dab_app_state(_dab_request.appId.clone())?;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
