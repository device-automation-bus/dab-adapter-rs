// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct GetAvailableLanguagesRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct GetAvailableLanguagesResponse{
// pub languages: Vec<String>,
// }

#[allow(unused_imports)]
use crate::dab::system::language::list::GetAvailableLanguagesRequest;
use crate::dab::system::language::list::GetAvailableLanguagesResponse;
#[allow(unused_imports)]
use crate::dab::ErrorResponse;
use crate::device::rdk::interface::get_rfc_5646_language_tag;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = GetAvailableLanguagesResponse::default();
    // *** Fill in the fields of the struct GetAvailableLanguagesResponse here ***

    ResponseOperator.languages = get_rfc_5646_language_tag();

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
