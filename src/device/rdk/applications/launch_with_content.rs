// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LaunchApplicationWithContentRequest{
// pub appId: String,
// pub contentId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LaunchApplicationWithContentResponse {}

use crate::dab::structs::ErrorResponse;
#[allow(unused_imports)]
use crate::dab::structs::LaunchApplicationWithContentRequest;
use crate::dab::structs::LaunchApplicationWithContentResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = LaunchApplicationWithContentResponse::default();
    // *** Fill in the fields of the struct LaunchApplicationWithContentResponse here ***

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

    let mut Dab_Request: LaunchApplicationWithContentRequest = IncomingMessage.unwrap();

    if Dab_Request.appId.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'appId' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    if !(Dab_Request.appId == "Cobalt"
        || Dab_Request.appId == "Youtube"
        || Dab_Request.appId == "YouTube")
    {
        let response = ErrorResponse {
            status: 500,
            error: "This operator currently only supports Youtube".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
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
    let mut is_suspended = false;
    for r in rdkresponse.result.state.iter() {
        let app = r.callsign.clone();
        if app == Dab_Request.appId {
            app_created = true;
            is_suspended = r.state == "suspended";
        }
    }
    let is_cobalt = Dab_Request.appId == "Cobalt"
        || Dab_Request.appId == "Youtube"
        || Dab_Request.appId == "YouTube";
    let mut param_list = vec![];
    if is_cobalt {
        if !(Dab_Request.contentId.is_empty()) {
            param_list.push(format!("v={}", Dab_Request.contentId.clone()));
        }
    }

    if let Some(mut parameters) = Dab_Request.parameters {
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
                params: Param,
            }

            // This is Cobalt only, we will need a switch statement for other apps.
            let req_params = Param {
                url: format!("https://www.YouTube.com/tv?{}", param_list.join("&")),
            };
            let request = RdkRequest {
                jsonrpc: "2.0".into(),
                id: 3,
                method: Dab_Request.appId.clone() + ".1.deeplink".into(),
                params: req_params,
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

            let rdkresponse: RdkResponseLaunch =
                serde_json::from_str(&response_json.unwrap()).unwrap();
            if rdkresponse.result.success == false {
                return Err("Error calling org.rdk.RDKShell.launch".to_string());
            }
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
                callsign: Dab_Request.appId,
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
            let response_json = http_post(json_string);

            match response_json {
                Err(err) => {
                    println!("Erro: {}", err);

                    return Err(err);
                }
                _ => (),
            }
            let rdkresponse: RdkResponseLaunch =
                serde_json::from_str(&response_json.unwrap()).unwrap();
            if rdkresponse.result.success == false {
                return Err("Error calling org.rdk.RDKShell.launch".to_string());
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
        let response_json = http_post(json_string);

        match response_json {
            Err(err) => {
                println!("Erro: {}", err);

                return Err(err);
            }
            _ => (),
        }
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
