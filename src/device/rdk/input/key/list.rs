// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct KeyListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct KeyList{
// pub keyCodes: Vec<String>,
// }

#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::KeyList;
#[allow(unused_imports)]
use crate::dab::structs::KeyListRequest;
use crate::device::rdk::interface::get_rdk_keys;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = KeyList::default();
    // *** Fill in the fields of the struct KeyList here ***

    ResponseOperator.keyCodes = get_rdk_keys();

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
