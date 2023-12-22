use super::voice_functions::sendVoiceCommand;
use crate::dab::structs::DabError;
use crate::dab::structs::SendAudioRequest;
use crate::device::rdk::interface::http_download;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: SendAudioRequest) -> Result<String, DabError> {
    http_download(_dab_request.fileLocation)?;
    sendVoiceCommand("/tmp/tts.wav".into())?;
    Ok("".into())
}
