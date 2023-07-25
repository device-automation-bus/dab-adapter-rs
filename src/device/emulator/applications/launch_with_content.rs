// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LaunchApplicationWithContentRequest{
// pub appId: String,
// pub contentId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LaunchApplicationWithContentResponse {}

#[allow(unused_imports)]
use crate::dab::structs::LaunchApplicationWithContentRequest;
use crate::dab::structs::LaunchApplicationWithContentResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = LaunchApplicationWithContentResponse::default();
    // *** Fill in the fields of the struct LaunchApplicationWithContentResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
