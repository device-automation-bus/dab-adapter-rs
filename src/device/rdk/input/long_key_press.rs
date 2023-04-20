// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct LongKeyPressRequest{
// pub keyCode: String,
// pub durationMs: u32,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct LongKeyPressResponse {}

use crate::dab::input::long_key_press::LongKeyPressRequest;
use crate::dab::input::long_key_press::LongKeyPressResponse;
use crate::dab::ErrorResponse;
use crate::device::rdk::interface::get_keycode;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = LongKeyPressResponse::default();
    // *** Fill in the fields of the struct LongKeyPressResponse here ***

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

    let Dab_Request: LongKeyPressRequest = IncomingMessage.unwrap();

    if Dab_Request.keyCode.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'keyCode' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    if Dab_Request.durationMs == 0 {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'durationMs' parameter".to_string(),
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

    //#########org.rdk.RDKShell.generateKey#########
    #[derive(Serialize)]
    struct GenerateKeyRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: GenerateKeyRequestParams,
    }

    #[derive(Serialize)]
    struct GenerateKeyRequestParams {
        keys: Vec<Key>,
    }

    #[derive(Serialize)]
    struct Key {
        keyCode: u16,
        modifiers: Vec<String>,
        delay: f32,
    }

    let key = Key {
        keyCode: KeyCode,
        modifiers: vec![],
        delay: (Dab_Request.durationMs as f32 / 1000.0),
    };

    let req_params = GenerateKeyRequestParams { keys: vec![key] };

    let request = GenerateKeyRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.1.generateKey".into(),
        params: req_params,
    };

    #[derive(Deserialize)]
    struct GenerateKeyResponse {
        jsonrpc: String,
        id: i32,
        result: GenerateKeyResult,
    }

    #[derive(Deserialize)]
    struct GenerateKeyResult {
        success: bool,
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
        _ => (),
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
