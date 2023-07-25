// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct SendTextRequest{
// pub requestText: String,
// pub voiceSystem: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct VoiceTextRequestResponse {}

#[allow(unused_imports)]
use crate::dab::structs::SendTextRequest;
use crate::dab::structs::VoiceTextRequestResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = VoiceTextRequestResponse::default();
    // *** Fill in the fields of the struct VoiceTextRequestResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
