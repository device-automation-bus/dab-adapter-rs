use crate::dab::structs::ErrorResponse;
use crate::dab::structs::SendAudioRequest;
use serde_json::json;
use super::voice_functions::sendVoiceCommand;
use crate::device::rdk::interface::http_download;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: SendAudioRequest) -> Result<String, String> {

    let result = http_download(_dab_request.fileLocation);
    match result {
        Ok(_) => {}
        Err(e) => {
            let response = ErrorResponse {
                status: 400,
                error: e,
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
    }

    sendVoiceCommand("/tmp/tts.wav".into())?;

    Ok("".into())
}
