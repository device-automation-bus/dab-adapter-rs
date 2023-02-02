// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct SetLanguageRequest{
// pub language: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct SetLanguageResponse {}

#[allow(unused_imports)]
use crate::dab::system::language::set::SetLanguageRequest;
use crate::dab::system::language::set::SetLanguageResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = SetLanguageResponse::default();
    // *** Fill in the fields of the struct SetLanguageResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
