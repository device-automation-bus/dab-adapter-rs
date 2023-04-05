// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SendAudioRequest{
// pub fileLocation: String,
// pub voiceSystem: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct VoiceAudioRequestResponse {}

#[allow(unused_imports)]
use crate::dab::voice::send_audio::SendAudioRequest;
use crate::dab::voice::send_audio::VoiceRequestResponse;
#[allow(unused_imports)]
use crate::dab::ErrorResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::voice_functions::encode_adpcm;
use super::voice_functions::sendVoiceCommamd;
use crate::device::rdk::interface::http_download;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(packet: String) -> Result<String, String> {
    use std::process::Command;
    use tts_rust::{languages::Languages, tts::GTTSClient};

    let IncomingMessage: Result<SendAudioRequest, serde_json::Error> =
        serde_json::from_str(&packet);

    match IncomingMessage {
        Err(err) => {
            let response = ErrorResponse {
                status: 400,
                error: "Error parsing request: ".to_string() + err.to_string().as_str(),
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
        Ok(DabRequest) => {
            if DabRequest.fileLocation.is_empty() {
                let response = ErrorResponse {
                    status: 400,
                    error: "request missing 'fileLocation' parameter".to_string(),
                };
                let Response_json = json!(response);
                return Err(serde_json::to_string(&Response_json).unwrap());
            }
            if let Err(e) = http_download(DabRequest.fileLocation) {
                let response = ErrorResponse {
                    status: 400,
                    error: e,
                };
                let Response_json = json!(response);
                return Err(serde_json::to_string(&Response_json).unwrap());
            }
        }
    }
    sendVoiceCommamd()
}
