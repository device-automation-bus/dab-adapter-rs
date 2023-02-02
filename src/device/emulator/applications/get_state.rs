// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct GetApplicationStateRequest{
// pub appId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct GetApplicationStateResponse{
// pub state: String,
// }

#[allow(unused_imports)]
use crate::dab::applications::get_state::GetApplicationStateRequest;
use crate::dab::applications::get_state::GetApplicationStateResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = GetApplicationStateResponse::default();
    // *** Fill in the fields of the struct GetApplicationStateResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
