// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct KeyPressRequest{
// pub keyCode: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct KeyPressResponse {}

#[allow(unused_imports)]
use crate::dab::structs::KeyPressRequest;
use crate::dab::structs::KeyPressResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = KeyPressResponse::default();
    // *** Fill in the fields of the struct KeyPressResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
