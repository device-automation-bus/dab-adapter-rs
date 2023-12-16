use super::voice_functions::configureVoice;
#[allow(unused_imports)]
use serde_json::json;
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::SetVoiceSystemRequest;
use crate::dab::structs::SetVoiceSystemResponse;


#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: SetVoiceSystemRequest) -> Result<String, String> {
    let mut ResponseOperator = SetVoiceSystemResponse::default();
    
    // TODO: Add other RDK specific voice protocol support confirmation.
    if _dab_request.voiceSystem.name != "AmazonAlexa" {
        // Unsupported VoiceSystem.
        let response = ErrorResponse {
            status: 400,
            error: "Setting voiceSystem failed. Unsupported voiceSystem.".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    configureVoice(_dab_request.voiceSystem.enabled)?;
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

    ResponseOperator.voiceSystem.enabled = _dab_request.voiceSystem.enabled;
    ResponseOperator.voiceSystem.name = _dab_request.voiceSystem.name;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
