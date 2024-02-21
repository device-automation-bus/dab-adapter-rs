use super::voice_functions::sendVoiceCommand;
use crate::dab::structs::DabError;
use crate::dab::structs::SendAudioRequest;
use crate::device::rdk::interface::http_download;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: SendAudioRequest) -> Result<String, DabError> {
    match let downloadresponse = http_download(_dab_request.fileLocation) {
        Ok(downloadresponse) => (),
        Err(e) => return e;
    }
    match let voiceresponse = sendVoiceCommand("/tmp/tts.wav".into()) {
        Ok(voiceresponse) => (),
        Err(e) => return e;
    }
    Ok("{}".to_string())
}
