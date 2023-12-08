// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SetVoiceSystemRequest{
// pub voiceSystem: VoiceSystem,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SetVoiceSystemResponse{
// pub voiceSystem: VoiceSystem,
// }

use super::voice_functions::configureVoice;
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
#[allow(unused_imports)]
use crate::dab::structs::SetVoiceSystemRequest;
use crate::dab::structs::SetVoiceSystemResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = SetVoiceSystemResponse::default();
    // *** parse and call configureVoice(arg)
    let IncomingMessage = serde_json::from_str(&_packet);

    match IncomingMessage {
        Err(_) => {
            let response = ErrorResponse {
                status: 400,
                error: "Setting voiceSystem failed. Argument parse failure.".to_string(),
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
        Ok(_) => (),
    }

    let Voice_Set_Request: SetVoiceSystemRequest = IncomingMessage.unwrap();

    if Voice_Set_Request.voiceSystem.name.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "Setting voiceSystem failed. Request missing parameter(s)".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    // TODO: Add other RDK specific voice protocol support confirmation.
    if Voice_Set_Request.voiceSystem.name != "AmazonAlexa" {
        // Unsupported VoiceSystem.
        let response = ErrorResponse {
            status: 400,
            error: "Setting voiceSystem failed. Unsupported voiceSystem.".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    configureVoice(Voice_Set_Request.voiceSystem.enabled)?;
    // TODO: validation of response.
    // if response.success == false {
    //     // Thunder JSONRPC failed
    //     let response = ErrorResponse {
    //         status: 400,
    //         error: "Platform operation failed.".to_string(),
    //     };
    //     let Response_json = json!(response);
    //     return Err(serde_json::to_string(&Response_json).unwrap());
    // }

    ResponseOperator.voiceSystem.enabled = Voice_Set_Request.voiceSystem.enabled;
    ResponseOperator.voiceSystem.name = Voice_Set_Request.voiceSystem.name;

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
