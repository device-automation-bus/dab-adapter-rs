// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct KeyListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct KeyList{
// pub keyCodes: Vec<String>,
// }

use crate::dab::input::key::list::KeyList;
#[allow(unused_imports)]
use crate::dab::input::key::list::KeyListRequest;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = KeyList::default();
    // *** Fill in the fields of the struct KeyList here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
