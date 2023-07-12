// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LaunchApplicationRequest{
// pub appId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LaunchApplicationResponse {}

#[allow(unused_imports)]
use crate::dab::applications::launch::LaunchApplicationRequest;
use crate::dab::applications::launch::LaunchApplicationResponse;
use crate::dab::ErrorResponse;
use crate::device::rdk::interface::http_post;
use serde::{Serialize,Deserialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = LaunchApplicationResponse::default();
    // *** Fill in the fields of the struct LaunchApplicationResponse here ***

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

    let Dab_Request: LaunchApplicationRequest = IncomingMessage.unwrap();

    if Dab_Request.appId.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'appId' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }


    // RDK Request Common Structs
    #[derive(Serialize,Clone)]
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

    let is_cobalt = Dab_Request.appId == "Cobalt" || Dab_Request.appId == "Youtube"
    let mut param_list = vec![]
    if (Dab_Request.parameters.len() > 0) {
        param_list.append(Dab_Request.parameters.clone())
    }
    
    if !app_created {
        if (is_cobalt) {
            // ****************** org.rdk.RDKShell.launch with Cobalt parameters ********************
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
                callsign: "Youtube".into(),
                r#type: Dab_Request.appId,
                configuration: CobaltConfig {
                    url: format!("https://www.YouTube.com/tv?{}", param_list.join("&")),
                },
            };
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
        } else {
            // ****************** org.rdk.RDKShell.launch ********************
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
            params: Param,
        }

        // This is Cobalt only, we will need a switch statement for other apps. 
        let req_params = Param {
            url: format!("https://www.YouTube.com/tv?{}", param_list.join("&")),
        };
        let request = RdkRequest {
            jsonrpc: "2.0".into(),
            id: 3,
            method: Dab_Request.appId.clone() + ".1.deeplink".into(),,
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
    }

    // ****************** org.rdk.RDKShell.moveToFront ********************

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.moveToFront".into(),
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

    // ****************** org.rdk.RDKShell.setFocus ********************

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.moveToFront".into(),
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

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
