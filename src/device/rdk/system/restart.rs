#[allow(unused_imports)]
use crate::dab::structs::RestartRequest;
use crate::dab::structs::RestartResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: RestartRequest) -> Result<String, String> {
    let mut ResponseOperator = RestartResponse::default();
    // *** Fill in the fields of the struct RestartResponse here ***

    //#########org.rdk.System.reboot#########
    #[derive(Serialize)]
    struct RebootRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: RebootRequestParams,
    }

    #[derive(Serialize)]
    struct RebootRequestParams {
        rebootReason: String,
    }

    let req_params = RebootRequestParams {
        rebootReason: "DAB_REBOOT_REQUEST".to_string(),
    };

    let request = RebootRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.System.reboot".into(),
        params: req_params,
    };

    #[derive(Deserialize)]
    struct RebootResponse {
        jsonrpc: String,
        id: i32,
        result: RebootResult,
    }

    #[derive(Deserialize)]
    struct RebootResult {
        IARM_Bus_Call_STATUS: u32,
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    http_post(json_string)?;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
