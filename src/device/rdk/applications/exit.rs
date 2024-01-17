// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ExitApplicationRequest{
// pub appId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ExitApplicationResponse{
// pub state: String,
// }

use crate::dab::structs::ErrorResponse;
#[allow(unused_imports)]
use crate::dab::structs::ExitApplicationRequest;
use crate::dab::structs::ExitApplicationResponse;
use crate::device::rdk::applications::get_state::get_app_state;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{thread, time};

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = ExitApplicationResponse::default();
    // *** Fill in the fields of the struct ExitApplicationResponse here ***

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

    let Dab_Request: ExitApplicationRequest = IncomingMessage.unwrap();

    if Dab_Request.appId.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'appId' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    let mut is_background = false;
    if Dab_Request.background.is_some() && Dab_Request.background.unwrap() {
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
        callsign: Dab_Request.appId.clone(),
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
    let response_json = http_post(json_string);

    match response_json {
        Err(err) => {
            println!("Erro: {}", err);

            return Err(err);
        }
        _ => (),
    }

    let rdkresponse: RdkResponseGetState = serde_json::from_str(&response_json.unwrap()).unwrap();
    let mut app_created = false;
    for r in rdkresponse.result.state.iter() {
        let app = r.callsign.clone();
        if app == Dab_Request.appId {
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
            let response_json = http_post(json_string);

            match response_json {
                Err(err) => {
                    println!("Erro: {}", err);

                    return Err(err);
                }
                _ => (),
            }
        } else {
            // ****************** org.rdk.RDKShell.destroy ********************
            let request = RdkRequest {
                jsonrpc: "2.0".into(),
                id: 3,
                method: "org.rdk.RDKShell.destroy".into(),
                params: req_params.clone(),
            };

            let json_string = serde_json::to_string(&request).unwrap();
            let response_json = http_post(json_string);

            match response_json {
                Err(err) => {
                    println!("Erro: {}", err);

                    return Err(err);
                }
                _ => (),
            }
        }
    }

    // ******************* wait until app state *************************
    for _idx in 1..=20 { // 2 seconds (20*100ms)
        // TODO: refactor to listen to Thunder events with websocket.
        thread::sleep(time::Duration::from_millis(100));
        ResponseOperator.state = get_app_state(Dab_Request.appId.clone())?;
        if (is_background && (ResponseOperator.state == "BACKGROUND".to_string()))
            || (!is_background && (ResponseOperator.state == "STOPPED".to_string()))
        {
            break;
        }
    }
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
