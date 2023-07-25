// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SendTextRequest{
// pub requestText: String,
// pub voiceSystem: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct VoiceTextRequestResponse {}

use crate::dab::structs::SendTextRequest;
use crate::dab::structs::ErrorResponse;
use serde_json::json;

use super::voice_functions::sendVoiceCommand;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(packet: String) -> Result<String, String> {
    use std::process::Command;
    use tts_rust::{languages::Languages, tts::GTTSClient};

    let IncomingMessage = serde_json::from_str(&packet);

    match IncomingMessage {
        Err(err) => {
            let response = ErrorResponse {
                status: 400,
                error: "Error parsing request: ".to_string() + err.to_string().as_str(),
            };
            let Response_json = json!(response);
            return Err(serde_json::to_string(&Response_json).unwrap());
        }
        _ => (),
    }

    let Dab_Request: SendTextRequest = IncomingMessage.unwrap();

    if Dab_Request.requestText.is_empty() {
        let response = ErrorResponse {
            status: 400,
            error: "request missing 'requestText' parameter".to_string(),
        };
        let Response_json = json!(response);
        return Err(serde_json::to_string(&Response_json).unwrap());
    }

    let narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English,
        tld: "com",
    };
    narrator
        .save_to_file(&Dab_Request.requestText, "/tmp/tts.mp3")
        .expect("Failed to save to file");

    let mut child = Command::new("gst-launch-1.0")
        .arg("-q")
        .arg("filesrc")
        .arg("location=/tmp/tts.mp3")
        .arg("!")
        .arg("decodebin")
        .arg("!")
        .arg("audioconvert")
        .arg("!")
        .arg("audioresample")
        .arg("!")
        .arg("audio/x-raw,rate=16000,channels=1,format=S16LE")
        .arg("!")
        .arg("wavenc")
        .arg("!")
        .arg("filesink")
        .arg("location=/tmp/tts.wav")
        .spawn()
        .expect("Failed to execute command");

    child.wait().expect("failed to wait for child process");

    return sendVoiceCommand();
}
