// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SetLanguageRequest{
	// pub language: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SetLanguageResponse {}

use serde::{Deserialize, Serialize};
use crate::device::rdk::interface::http_post;
use serde_json::json;
# [allow(unused_imports)]
use crate::dab::ErrorResponse;
# [allow(unused_imports)]
use crate::dab::system::language::set::SetLanguageRequest;
use crate::dab::system::language::set::SetLanguageResponse;

# [allow(non_snake_case)]
# [allow(dead_code)]
# [allow(unused_mut)]
pub fn process (_packet: String) -> Result<String,String> {
	let mut ResponseOperator = SetLanguageResponse::default();
	// *** Fill in the fields of the struct SetLanguageResponse here ***
	
	#[derive(Serialize)]
	struct RdkRequest {
		jsonrpc: String,
		id: i32,
		method: String,
		params: String,
	}

	
	let request = RdkRequest {
		        jsonrpc : "2.0".into(),
		        id      : 3,
		        method  : "org.rdk.DisplaySettings.getConnectedVideoDisplays".into(),
		        params  : "{}".into(),

	    };

	
	#[derive(Deserialize)]
	struct RdkResponse {
		jsonrpc: String,
		id: i32,
		result: GetConnectedVideoDisplaysResult,
	}

	
	#[derive(Deserialize)]
	struct GetConnectedVideoDisplaysResult {
		connectedVideoDisplays: Vec<String>,
		success: bool,
	}

	
	    let json_string = serde_json::to_string(&request).unwrap();
	    let response_json = http_post(json_string);
	
	    match response_json {
		        Ok(val2) => {
			let rdkresponse: RdkResponse = serde_json::from_str(&val2).unwrap();
			            println!("Sucesso: {}", rdkresponse.result.success);

		        }

		        Err(err) => {
			            println!("Erro: {}", err);

			return Err(err)
		        }

	    }

	
	// *******************************************************************
	let mut ResponseOperator_json = json!(ResponseOperator);
	ResponseOperator_json["status"] = json!(200);
	Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}

