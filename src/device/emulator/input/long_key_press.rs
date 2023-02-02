// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LongKeyPressRequest{
// pub keyCode: String,
// pub durationMs: u32,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct KeyPressResponse {}

use crate::dab::input::long_key_press::KeyPressResponse;
#[allow(unused_imports)]
use crate::dab::input::long_key_press::LongKeyPressRequest;
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
