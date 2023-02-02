// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct GetLanguageRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct GetLanguageResponse{
// pub language: String,
// }

#[allow(unused_imports)]
use crate::dab::system::language::get::GetLanguageRequest;
use crate::dab::system::language::get::GetLanguageResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = GetLanguageResponse::default();
    // *** Fill in the fields of the struct GetLanguageResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
