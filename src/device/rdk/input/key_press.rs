use crate::dab::structs::DabError;
use crate::dab::structs::KeyPressRequest;
use crate::dab::structs::KeyPressResponse;
use crate::device::rdk::interface::get_keycode;
use crate::device::rdk::connectivity::http::http_post;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: KeyPressRequest) -> Result<String, DabError> {
    let mut ResponseOperator = KeyPressResponse::default();
    // *** Fill in the fields of the struct KeyPressResponse here ***

    if _dab_request.keyCode.is_empty() {
        return Err(DabError::Err400(
            "request missing 'keyCode' parameter".to_string(),
        ));
    }

    let mut KeyCode: u16;

    match get_keycode(_dab_request.keyCode.clone()) {
        Some(k) => KeyCode = *k,
        None => return Err(DabError::Err400("keyCode' not found".to_string())),
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
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
