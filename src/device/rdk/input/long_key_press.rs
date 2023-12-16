use crate::dab::structs::ErrorResponse;
use crate::dab::structs::LongKeyPressRequest;
use crate::dab::structs::LongKeyPressResponse;
use crate::device::rdk::interface::get_keycode;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{self, Value};
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: LongKeyPressRequest) -> Result<String, String> {
    let mut ResponseOperator = LongKeyPressResponse::default();
    // *** Fill in the fields of the struct LongKeyPressResponse here ***

    if _dab_request.durationMs == 0 {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'durationMs' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    let mut KeyCode: u16;

    match get_keycode(_dab_request.keyCode.clone()) {
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
        keys: Value,
    }

    let interval_ms: u64 = 50;
    let total_time = _dab_request.durationMs;

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
    let mut elapsed_time = 0;

    while elapsed_time < total_time {
        let start_time = Instant::now();

        http_post(json_string.clone())?;

        let mut end_time = Instant::now().duration_since(start_time).as_millis();
        if end_time < interval_ms.into() {
            thread::sleep(Duration::from_millis(interval_ms - end_time as u64));
            end_time = Instant::now().duration_since(start_time).as_millis();
        }

        elapsed_time += end_time as u32;
    }

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
