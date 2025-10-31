use super::voice_functions::sendVoiceCommand;
use crate::dab::structs::DabError;
use crate::dab::structs::SendAudioRequest;
use crate::device::rdk::interface::http::http_download;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: SendAudioRequest) -> Result<String, DabError> {
    if _dab_request.voiceSystem.to_string() != "AmazonAlexa" {
        return Err(DabError::Err400("Unsupported 'voiceSystem'.".to_string()));
    }

    http_download(_dab_request.fileLocation)?;
    sendVoiceCommand("/tmp/tts.wav".into())?;
    Ok("{}".to_string())
}
