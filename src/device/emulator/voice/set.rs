// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct SetVoiceSystemRequest{
// pub voiceSystem: VoiceSystem,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct SetVoiceSystemResponse{
// pub voiceSystem: VoiceSystem,
// }

#[allow(unused_imports)]
use crate::dab::structs::SetVoiceSystemRequest;
use crate::dab::structs::SetVoiceSystemResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = SetVoiceSystemResponse::default();
    // *** Fill in the fields of the struct SetVoiceSystemResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
