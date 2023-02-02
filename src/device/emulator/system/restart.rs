// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct RestartRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct RestartResponse {}

#[allow(unused_imports)]
use crate::dab::system::restart::RestartRequest;
use crate::dab::system::restart::RestartResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = RestartResponse::default();
    // *** Fill in the fields of the struct RestartResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
