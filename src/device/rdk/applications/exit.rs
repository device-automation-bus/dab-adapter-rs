use crate::dab::structs::DabError;
use crate::dab::structs::ExitApplicationRequest;
use crate::dab::structs::ExitApplicationResponse;
use crate::device::rdk::applications::get_state::get_app_state;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::interface::get_lifecycle_timeout;
use serde::{Deserialize, Serialize};
use std::{thread, time};

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: ExitApplicationRequest) -> Result<String, DabError> {
    let mut ResponseOperator = ExitApplicationResponse::default();
    // *** Fill in the fields of the struct ExitApplicationResponse here ***
    if _dab_request.appId.is_empty() {
        return Err(DabError::Err400(
            "request missing 'appId' parameter".to_string(),
        ));
    }

    let mut is_background = false;
    if _dab_request.background.is_some() && _dab_request.background.unwrap() {
        is_background = true;
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
    for r in rdkresponse.result.state.iter() {
        let app = r.callsign.clone();
        if app == _dab_request.appId {
            app_created = true;
        }
    }

    if app_created {
        if is_background {
            // ****************** org.rdk.RDKShell.suspend ********************
            let request = RdkRequest {
                jsonrpc: "2.0".into(),
                id: 3,
                method: "org.rdk.RDKShell.suspend".into(),
                params: req_params.clone(),
            };

            let json_string = serde_json::to_string(&request).unwrap();
            http_post(json_string)?;
        } else {
            // ****************** org.rdk.RDKShell.destroy ********************
            let request = RdkRequest {
                jsonrpc: "2.0".into(),
                id: 3,
                method: "org.rdk.RDKShell.destroy".into(),
                params: req_params.clone(),
            };

            let json_string = serde_json::to_string(&request).unwrap();
            http_post(json_string)?;
        }
    }

    // *******************************************************************
    for _idx in 1..=20 {
        // 2 seconds (20*100ms)
        // TODO: refactor to listen to Thunder events with websocket.
        thread::sleep(time::Duration::from_millis(100));
        ResponseOperator.state = get_app_state(_dab_request.appId.clone())?;
        if (is_background && (ResponseOperator.state == "BACKGROUND".to_string()))
            || (!is_background && (ResponseOperator.state == "STOPPED".to_string()))
        {
            let timeout_type = if is_background {
                "exit_to_background_timeout_ms"
            } else {
                "exit_to_destroy_timeout_ms"
            };
            
            let sleep_time = get_lifecycle_timeout(&_dab_request.appId.to_lowercase(), timeout_type).unwrap_or(2500);
            std::thread::sleep(time::Duration::from_millis(sleep_time));
            break;
        }
    }
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
