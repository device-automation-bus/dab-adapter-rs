// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ExitApplicationRequest{
// pub appId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ExitApplicationResponse{
// pub state: String,
// }

#[allow(unused_imports)]
use crate::dab::structs::ExitApplicationRequest;
use crate::dab::structs::ExitApplicationResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = ExitApplicationResponse::default();
    // *** Fill in the fields of the struct ExitApplicationResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
