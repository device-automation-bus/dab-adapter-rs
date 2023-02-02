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

#[allow(unused_imports)]
use crate::dab::applications::exit::ExitApplicationRequest;
use crate::dab::applications::exit::ExitApplicationResponse;
use crate::dab::ErrorResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
        Ok(_) => (),
    }

    let Dab_Request: ExitApplicationRequest = IncomingMessage.unwrap();

    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: RequestParams,
    }

    #[derive(Serialize)]
    struct RequestParams {
        callsign: String,
    }

    if Dab_Request.appId.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'appId' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    let req_params = RequestParams {
        callsign: Dab_Request.appId.clone(),
    };

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.destroy".into(),
        params: req_params,
    };

    #[derive(Deserialize)]
    struct RdkResponse {
        jsonrpc: String,
        id: i32,
        result: DestroyResult,
    }

    #[derive(Deserialize)]
    struct DestroyResult {
        success: bool,
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
    struct GetStateResultGetState {
        state: Vec<State>,
        success: bool,
    }

    #[derive(Deserialize)]
    struct RdkResponseGetState {
        jsonrpc: String,
        id: i32,
        result: GetStateResultGetState,
    }

    #[derive(Deserialize)]
    struct State {
        callsign: String,
        state: String,
        uri: String,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(response) => {
            let rdkresponse: RdkResponseGetState = serde_json::from_str(&response).unwrap();

            for item in rdkresponse.result.state {
                if item.callsign != Dab_Request.appId {
                    continue;
                }
                match item.state.as_str() {
                    "suspended" => ResponseOperator.state = "BACKGROUND".to_string(),
                    _ => ResponseOperator.state = "FOREGROUND".to_string(),
                }
                break;
            }

            // We couldn't find the requested appId in the list, that
            // means the app isn't running yet
            if ResponseOperator.state.is_empty() {
                ResponseOperator.state = "STOPPED".to_string();
            }
        }
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
