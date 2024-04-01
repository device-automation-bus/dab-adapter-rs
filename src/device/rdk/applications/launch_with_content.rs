use crate::dab::structs::DabError;
use crate::dab::structs::LaunchApplicationWithContentRequest;
use crate::dab::structs::LaunchApplicationWithContentResponse;
use crate::device::rdk::applications::get_state::get_app_state;
use crate::device::rdk::applications::launch::move_to_front_set_focus;
use crate::device::rdk::applications::launch::RDKShellParams;
use crate::device::rdk::applications::launch::send_rdkshell_launch_request;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::interface::get_lifecycle_timeout;
use crate::hw_specific::applications::launch::get_visibility;
use crate::hw_specific::applications::launch::set_visibility;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;

use std::{thread, time};
use urlencoding::decode;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: LaunchApplicationWithContentRequest) -> Result<String, DabError> {
    let mut ResponseOperator = LaunchApplicationWithContentResponse::default();
    // *** Fill in the fields of the struct LaunchApplicationWithContentResponse here ***

    if _dab_request.appId.is_empty() {
        return Err(DabError::Err400(
            "request missing 'appId' parameter".to_string(),
        ));
    }

    if !(_dab_request.appId == "Cobalt"
        || _dab_request.appId == "Youtube"
        || _dab_request.appId == "YouTube")
    {
        return Err(DabError::Err400(
            "This operator currently only supports Youtube".to_string(),
        ));
    }

    // ****** RDK Request Common Structs ********

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
    let mut param_list = vec![];
    if is_cobalt {
        if !(_dab_request.contentId.is_empty()) {
            param_list.push(format!("v={}", _dab_request.contentId.clone()));
        }
    }

    if let Some(mut parameters) = _dab_request.parameters {
        if is_cobalt {
            // Decode each parameter before appending to the list
            for param in &mut parameters {
                *param = decode(param).unwrap().to_string();
            }
        }
        param_list.append(&mut parameters);
    }

    if app_created {
        if is_cobalt {
            // ****************** Youtube.1.deeplink ********************
            #[derive(Serialize)]
            struct RdkRequest {
                jsonrpc: String,
                id: i32,
                method: String,
                params: String,
            }
            let request = RdkRequest {
                jsonrpc: "2.0".into(),
                id: 3,
                method: _dab_request.appId.clone() + ".1.deeplink".into(),
                params: format!("https://www.youtube.com/tv?{}", param_list.join("&")),
            };
            let json_string = serde_json::to_string(&request).unwrap();
            http_post(json_string)?;
        }
        // TODO: Add other apps here
        if is_suspended {
            // RDKShell.launch will resume the app if it is suspended.
            let req_params = RDKShellParams {
                callsign: _dab_request.appId.clone(),
                r#type: "Cobalt".into(),
                configuration: None,
            };
            send_rdkshell_launch_request(req_params)?;
        }
        //****************org.rdk.RDKShell.moveToFront/setFocus******************************//
        move_to_front_set_focus(req_params.callsign.clone())?;
        if !get_visibility(req_params.callsign.clone())? {
            set_visibility(req_params.callsign.clone(), true)?;
        }
    } else {
        // Cold launch
        // ****************** org.rdk.RDKShell.launch ******************** //
        let req_params = if is_cobalt {
            let url = format!("https://www.youtube.com/tv?{}", param_list.join("&"));
            let config = json!({"url": url});
            RDKShellParams {
                callsign: _dab_request.appId.clone(),
                r#type: "Cobalt".into(),
                configuration: Some(config.to_string()),
            }
        } else {
            // Common webapp?. URL is not provided in the request; hence do not override.
            RDKShellParams {
                callsign: _dab_request.appId.clone(),
                r#type: "LightningApp".into(),
                configuration: None,
            }
        };
        send_rdkshell_launch_request(req_params)?;
    }

    // ******************* wait until app state 8*************************
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
