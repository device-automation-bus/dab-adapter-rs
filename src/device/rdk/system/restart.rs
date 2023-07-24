// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct RestartRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct RestartResponse {}

#[allow(unused_imports)]
use crate::dab::structs::RestartRequest;
use crate::dab::structs::RestartResponse;
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
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
    let response_json = http_post(json_string);

    match response_json {
        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
        Ok(_) => {}
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
