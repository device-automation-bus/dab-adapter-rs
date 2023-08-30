// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ApplicationListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct Application{
// pub appId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default, Serialize)]
// ListApplicationsResponse{
//     pub applications: Vec<Application>,
// }

use crate::dab::structs::Application;
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::ListApplicationsResponse;
#[allow(unused_imports)]
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = ListApplicationsResponse::default();
    // *** Fill in the fields of the struct Application here ***

    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: String,
    }

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.getAvailableTypes".into(),
        params: "{}".into(),
    };

    #[derive(Deserialize)]
    struct RdkResponse {
        jsonrpc: String,
        id: i32,
        result: GetAvailableTypesResult,
    }

    #[derive(Deserialize)]
    struct GetAvailableTypesResult {
        types: Vec<String>,
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    match response_json {
        Ok(val2) => {
            let rdkresponse: RdkResponse = serde_json::from_str(&val2).unwrap();
            for s in rdkresponse.result.types.iter() {
                let app = Application { appId: s.clone() };
                ResponseOperator.applications.push(app);
            }
        }

        Err(err) => {
            let error = ErrorResponse {
                status: 500,
                error: err,
            };
            return Err(serde_json::to_string(&error).unwrap());
        }
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
