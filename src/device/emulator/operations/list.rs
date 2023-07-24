// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct OperationsListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct ListSupportedOperation{
// pub operations: Vec<String>,
// }

use crate::dab::structs::ListSupportedOperation;
#[allow(unused_imports)]
use crate::dab::structs::OperationsListRequest;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = ListSupportedOperation::default();
    // *** Fill in the fields of the struct ListSupportedOperation here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
