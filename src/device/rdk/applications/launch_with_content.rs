use crate::dab::structs::DabError;
use crate::dab::structs::LaunchApplicationWithContentRequest;
use crate::dab::structs::LaunchApplicationWithContentResponse;
use crate::device::rdk::applications::get_state::get_app_state;
use crate::device::rdk::applications::launch::move_to_front_set_focus;
use crate::device::rdk::interface::http_post;
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

    if is_cobalt {
        if app_created {
            // ****************** Youtube.1.deeplink ********************
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
        } else {
            // ****************** org.rdk.RDKShell.launch ********************
            #[derive(Serialize)]
            struct CobaltConfig {
                url: String,
            }
            #[derive(Serialize)]
            struct Param {
                callsign: String,
                r#type: String,
                configuration: CobaltConfig,
            }
            #[derive(Serialize)]
            struct RdkRequest {
                jsonrpc: String,
                id: i32,
                method: String,
                params: Param,
            }

            let req_params = Param {
                callsign: _dab_request.appId,
                r#type: "Cobalt".into(),
                configuration: CobaltConfig {
                    url: format!("https://www.YouTube.com/tv?{}", param_list.join("&")),
                },
            };
            let request = RdkRequest {
                jsonrpc: "2.0".into(),
                id: 3,
                method: "org.rdk.RDKShell.launch".into(),
                params: req_params,
            };
            let json_string = serde_json::to_string(&request).unwrap();
            let response = http_post(json_string)?;
            let rdkresponse: RdkResponseLaunch = serde_json::from_str(&response).unwrap();
            if rdkresponse.result.success == false {
                return Err(DabError::Err500(
                    "Error calling org.rdk.RDKShell.launch".to_string(),
                ));
            }
        }
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
    }

    // ******************* wait until app state 8*************************
    let mut app_state: String = "STOPPED".to_string();
    for _idx in 1..=20 {
        // 5 seconds (20*250ms)
        // TODO: refactor to listen to Thunder events with websocket.
        thread::sleep(time::Duration::from_millis(250));
        app_state = get_app_state(req_params.callsign.clone())?;
        if app_state == "FOREGROUND".to_string() {
            if !app_created {
                // Worst case to launch and complete the initialization of plugin runtime and App SDK.
                // TODO: Temporary solution; will be replaced by event listener when plugin shares apt event.
                thread::sleep(time::Duration::from_millis(2000));
            } else {
                // Worst case to resume the plugin runtime and App SDK.
                thread::sleep(time::Duration::from_millis(250));
            }
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
