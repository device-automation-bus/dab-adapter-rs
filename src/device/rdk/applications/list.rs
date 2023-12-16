use crate::dab::structs::Application;
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
    let response = http_post(json_string)?;

    let rdkresponse: RdkResponse = serde_json::from_str(&response).unwrap();
    for s in rdkresponse.result.types.iter() {
        match s.as_str() {
            "YouTube" => {
                let app = Application {
                    appId: ("YouTube").to_string(),
                };
                ResponseOperator.applications.push(app);
            }
            "Amazon" => {
                let app = Application {
                    appId: ("PrimeVideo").to_string(),
                };
                ResponseOperator.applications.push(app);
            }
            "Netflix" => {
                let app = Application {
                    appId: ("Netflix").to_string(),
                };
                ResponseOperator.applications.push(app);
            }
            &_ => println!("Out of scope of DAB2.0 Spec."),
        }
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
