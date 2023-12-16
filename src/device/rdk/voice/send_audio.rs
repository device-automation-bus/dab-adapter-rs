// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SendAudioRequest{
// pub fileLocation: String,
// pub voiceSystem: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct VoiceAudioRequestResponse {}

use crate::dab::structs::ErrorResponse;
use crate::dab::structs::SendAudioRequest;
use serde_json::json;

use super::voice_functions::convert_audio_to_pcms16le16mono;
use super::voice_functions::is_supported_audio_format;
use super::voice_functions::sendVoiceCommand;
use crate::device::rdk::interface::http_download;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(packet: String) -> Result<String, String> {
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
            let result = http_download(DabRequest.fileLocation, "/tmp/tts-download.wav".into());
            match result {
                Ok(_) => {
                    // RDK Currently supports PCM S16LE 16K 256kbps. Convert on mismatch.
                    if !is_supported_audio_format("/tmp/tts-download.wav".into()) {
                        println!("Calling convert_audio_to_pcms16le16mono");
                        convert_audio_to_pcms16le16mono(
                            "/tmp/tts-download.wav".into(),
                            "/tmp/tts.wav".into(),
                        );
                    }
                }
                Err(e) => {
                    let response = ErrorResponse {
                        status: 400,
                        error: e,
                    };
                    let Response_json = json!(response);
                    return Err(serde_json::to_string(&Response_json).unwrap());
                }
            }
        }
    }

    sendVoiceCommand("/tmp/tts.wav".into())?;

    Ok(serde_json::to_string(&json!({"status": 200})).unwrap())
}
