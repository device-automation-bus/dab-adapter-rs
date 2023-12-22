use super::voice_functions::configureVoice;
use crate::dab::structs::DabError;
use crate::dab::structs::SetVoiceSystemRequest;
use crate::dab::structs::SetVoiceSystemResponse;


#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: SetVoiceSystemRequest) -> Result < String, DabError > {
    let mut ResponseOperator = SetVoiceSystemResponse::default();
    
    // TODO: Add other RDK specific voice protocol support confirmation.
    if _dab_request.voiceSystem.name != "AmazonAlexa" {
        // Unsupported VoiceSystem.
            return Err(DabError::Err400("Setting voiceSystem failed. Unsupported voiceSystem.".to_string()));
    }

    configureVoice(_dab_request.voiceSystem.enabled)?;
    // TODO: validation of response.
    // if response.success == false {
    //     return Err(DabError::Err400("Platform operation failed.".to_string());
    // }

    ResponseOperator.voiceSystem.enabled = _dab_request.voiceSystem.enabled;
    ResponseOperator.voiceSystem.name = _dab_request.voiceSystem.name;

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
