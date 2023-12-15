// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct KeyPressRequest{
// pub keyCode: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct KeyPressResponse {}

#[allow(unused_imports)]
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::KeyPressRequest;
use crate::dab::structs::KeyPressResponse;
use crate::device::rdk::interface::get_keycode;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = KeyPressResponse::default();
    // *** Fill in the fields of the struct KeyPressResponse here ***

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

    let Dab_Request: KeyPressRequest = IncomingMessage.unwrap();

    if Dab_Request.keyCode.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'keyCode' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    let mut KeyCode: u16;

    match get_keycode(Dab_Request.keyCode.clone()) {
        Some(k) => KeyCode = *k,
        None => {
            let response = ErrorResponse {
                status: 400,
                error: "keyCode' not found".to_string(),
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
    }

    //#########org.rdk.RDKShell.injectKey#########
    #[derive(Serialize)]
    struct InjectKeyRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: InjectKeyRequestParams,
    }

    #[derive(Serialize)]
    struct InjectKeyRequestParams {
        keyCode: u16,
    }

    let req_params = InjectKeyRequestParams { keyCode: KeyCode };

    let request = InjectKeyRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.1.injectKey".into(),
        params: req_params,
    };

    #[derive(Deserialize)]
    struct InjectKeyResponse {
        jsonrpc: String,
        id: i32,
        result: InjectKeyResult,
    }

    #[derive(Deserialize)]
    struct InjectKeyResult {
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    http_post(json_string)?;

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
