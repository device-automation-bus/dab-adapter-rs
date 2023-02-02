// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct SendAudioRequest{
// pub fileLocation: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct VoiceRequestResponse {}

#[allow(unused_imports)]
use crate::dab::voice::send_audio::SendAudioRequest;
use crate::dab::voice::send_audio::VoiceRequestResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let ResponseOperator = VoiceRequestResponse::default();
    // *** Fill in the fields of the struct VoiceRequestResponse here ***

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
