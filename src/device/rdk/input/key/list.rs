use crate::dab::structs::DabError;
use crate::dab::structs::KeyList;
use crate::dab::structs::KeyListRequest;
use crate::device::rdk::interface::get_rdk_keys;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: KeyListRequest) -> Result<String, DabError> {
    let mut ResponseOperator = KeyList::default();
    // *** Fill in the fields of the struct KeyList here ***

    ResponseOperator.keyCodes = get_rdk_keys();

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
