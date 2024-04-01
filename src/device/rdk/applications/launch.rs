use crate::dab::structs::DabError;
use crate::dab::structs::LaunchApplicationRequest;
use crate::dab::structs::LaunchApplicationResponse;
use crate::device::rdk::applications::get_state::get_app_state;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::interface::get_lifecycle_timeout;
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::{thread, time};
use urlencoding::decode;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: LaunchApplicationRequest) -> Result<String, DabError> {
    let mut ResponseOperator = LaunchApplicationResponse::default();
    // *** Fill in the fields of the struct LaunchApplicationResponse here ***
    if _dab_request.appId.is_empty() {
        return Err(DabError::Err400(
            "request missing 'appId' parameter".to_string(),
        ));
    }

    // RDK Request Common Structs
    #[derive(Serialize, Clone)]
    struct RequestParams {
        callsign: String,
    }

    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: RequestParams,
    }

    let req_params = RequestParams {
        callsign: _dab_request.appId.clone(),
    };
    // ****************** org.rdk.RDKShell.getState ********************
    #[derive(Serialize)]
    struct RdkRequestGetState {
        jsonrpc: String,
        id: i32,
        method: String,
    }

    let request = RdkRequestGetState {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.getState".into(),
    };

    #[derive(Deserialize)]
    struct Runtimes {
        callsign: String,
        state: String,
        uri: String,
        lastExitReason: i32,
    }

    #[derive(Deserialize)]
    struct GetStateResult {
        state: Vec<Runtimes>,
        success: bool,
    }

    #[derive(Deserialize)]
    struct RdkResponseGetState {
        jsonrpc: String,
        id: i32,
        result: GetStateResult,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response = http_post(json_string)?;

    let rdkresponse: RdkResponseGetState = serde_json::from_str(&response).unwrap();
    let mut app_created = false;
    let mut is_suspended = false;
    for r in rdkresponse.result.state.iter() {
        let app = r.callsign.clone();
        if app == _dab_request.appId {
            app_created = true;
            is_suspended = r.state == "suspended";
        }
    }

    let is_cobalt = _dab_request.appId == "Cobalt"
        || _dab_request.appId == "Youtube"
        || _dab_request.appId == "YouTube";
    let is_netflix = _dab_request.appId == "Netflix";
    let mut param_list = vec![];
    if let Some(mut parameters) = _dab_request.parameters.clone() {
        if parameters.len() > 0 {
            if is_cobalt {
                // Decode each parameter before appending to the list
                for param in &mut parameters {
                    *param = decode(param).unwrap().to_string();
                }
            }
            param_list.append(&mut parameters);
        }
    }

    if !app_created {
        let req_params = if is_cobalt {
            let url = format!("https://www.youtube.com/tv?{}", param_list.join("&"));
            let config = json!({"url": url});
            RDKShellParams {
                callsign: _dab_request.appId,
                r#type: "Cobalt".into(),
                configuration: Some(config.to_string()),
            }
        } else if is_netflix {
            let querystring = format!("{}", param_list.join("&"));
            let config = json!({"querystring": querystring});
            RDKShellParams {
                callsign: _dab_request.appId,
                r#type: "Netflix".into(),
                configuration: Some(config.to_string()),
            }
        } else {
            RDKShellParams {
                callsign: _dab_request.appId,
                r#type: "LightningApp".into(),
                configuration: None,
            }
        };
        send_rdkshell_launch_request(req_params)?;
    } else {
        // ****************** Cobalt.1.deeplink ********************
        #[derive(Serialize)]
        struct Param {
            url: String,
        }
        #[derive(Serialize)]
        struct RdkRequest {
            jsonrpc: String,
            id: i32,
            method: String,
            params: String,
        }

        // This is Cobalt only, we will need a switch statement for other apps.
        let request = RdkRequest {
            jsonrpc: "2.0".into(),
            id: 3,
            method: _dab_request.appId.clone() + ".1.deeplink".into(),
            params: format!("https://www.youtube.com/tv?{}", param_list.join("&")),
        };
        let json_string = serde_json::to_string(&request).unwrap();
        http_post(json_string)?;

        //****************org.rdk.RDKShell.moveToFront/setFocus******************************//
        move_to_front_set_focus(req_params.callsign.clone())?;
    }

    if is_suspended {
        // ****************** org.rdk.RDKShell.resumeApplication ********************
        let request = RdkRequest {
            jsonrpc: "2.0".into(),
            id: 3,
            method: "org.rdk.RDKShell.launch".into(),
            params: req_params.clone(),
        };

        let json_string = serde_json::to_string(&request).unwrap();
        http_post(json_string)?;
        //****************org.rdk.RDKShell.moveToFront/setFocus******************************//
        move_to_front_set_focus(req_params.callsign.clone())?;
        if !get_visibility(req_params.callsign.clone())? {
            set_visibility(req_params.callsign.clone(), true)?;
        }
    }

    // ******************* wait until app state *************************
    let mut app_state: String = "STOPPED".to_string();
    for _idx in 1..=20 {
        // 5 seconds (20*250ms)
        // TODO: refactor to listen to Thunder events with websocket.
        thread::sleep(time::Duration::from_millis(250));
        app_state = get_app_state(req_params.callsign.clone())?;
        if app_state == "FOREGROUND".to_string() {
            let timeout_type = if !app_created {
                "cold_launch_timeout_ms"
            } else {
                "resume_launch_timeout_ms"
            };

            let sleep_time = get_lifecycle_timeout(&req_params.callsign.to_lowercase(), timeout_type).unwrap_or(2500);
            // TODO: Temporary solution; will be replaced by event listener when plugin shares apt event.
            std::thread::sleep(time::Duration::from_millis(sleep_time));
            break;
        }
    }

    if app_state != "FOREGROUND" {
        return Err(DabError::Err500(
            "Check state request(5 second) timeout, app may not be visible to user.".to_string(),
        ));
    }

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

// Make these structs public

#[derive(Serialize, Clone)]
pub struct RDKShellParams {
    pub callsign: String,
    pub r#type: String,
    pub configuration: Option<String>,
}

#[derive(Serialize)]
pub struct RdkRequestWithParamConfig {
    pub jsonrpc: String,
    pub id: i32,
    pub method: String,
    pub params: RDKShellParams,
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn send_rdkshell_launch_request(params: RDKShellParams) -> Result<(), DabError> {    
    #[derive(Deserialize)]
    struct LaunchResult {
        launchType: String,
        success: bool,
    }

    #[derive(Deserialize)]
    struct RdkResponseLaunch {
        jsonrpc: String,
        id: i32,
        result: LaunchResult,
    }

    let request = RdkRequestWithParamConfig {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.launch".into(),
        params,
    };
    let json_string = serde_json::to_string(&request).unwrap();
    let response = http_post(json_string)?;
    let rdkresponse: RdkResponseLaunch = serde_json::from_str(&response).unwrap();
    if rdkresponse.result.success == false {
        return Err(DabError::Err500(
            "Error calling org.rdk.RDKShell.launch".to_string(),
        ));
    }
    Ok(())
}

pub fn move_to_front_set_focus(callsign: String) -> Result<String, DabError> {
    //****************org.rdk.RDKShell.moveToFront/setFocus******************************//

    // RDK Request Common Structs
    #[derive(Serialize, Clone)]
    struct RequestParams {
        client: String,
        callsign: String,
    }

    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: RequestParams,
    }

    let req_params = RequestParams {
        client: callsign.clone(),
        callsign: callsign.clone(),
    };
    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.moveToFront".into(),
        params: req_params.clone(),
    };
    let json_string = serde_json::to_string(&request).unwrap();
    http_post(json_string)?;

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.1.setFocus".into(),
        params: req_params.clone(),
    };
    let json_string = serde_json::to_string(&request).unwrap();
    http_post(json_string)?;
    Ok("{}".to_string())
}

pub fn set_visibility(client: String, visible: bool) -> Result<String, DabError> {
    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: RequestParams,
    }

    #[derive(Serialize)]
    struct RequestParams {
        client: String,
        callsign: String,
        visible: bool,
    }

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.setVisibility".into(),
        params: RequestParams {
            client: client.clone(),
            callsign: client,
            visible: visible,
        },
    };

    let json_string = serde_json::to_string(&request).unwrap();
    http_post(json_string)?;
    Ok("{}".to_string())
}

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn get_visibility(client: String) -> Result<bool, DabError> {
    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: RequestParams,
    }

    #[derive(Serialize)]
    struct RequestParams {
        client: String,
        callsign: String,
    }

    #[derive(Deserialize)]
    struct VisibilityResult {
        visible: bool,
        success: bool,
    }

    #[derive(Deserialize)]
    struct RdkResponse {
        jsonrpc: String,
        id: i32,
        result: VisibilityResult,
    }

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.getVisibility".into(),
        params: RequestParams {
            client: client.clone(),
            callsign: client.clone(),
        },
    };

    let json_string = serde_json::to_string(&request).unwrap();
    let response = http_post(json_string)?;
    let rdkresponse: RdkResponse = serde_json::from_str(&response).unwrap();
    Ok(rdkresponse.result.visible)
}
