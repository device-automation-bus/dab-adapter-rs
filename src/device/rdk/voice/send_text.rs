use crate::dab::structs::DabError;
use crate::dab::structs::SendTextRequest;
use super::voice_functions::sendVoiceCommand;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: SendTextRequest) -> Result < String, DabError > {
    use std::process::Command;
    use tts_rust::{languages::Languages, tts::GTTSClient};

    // TODO: Add other RDK specific voice protocol support confirmation.
    if _dab_request.voiceSystem.to_string() != "AmazonAlexa" {
        return Err(DabError::Err400("Unsupported 'voiceSystem'.".to_string()));
    }

    let narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English,
        tld: "com",
    };
    narrator
        .save_to_file(&_dab_request.requestText, "/tmp/tts.mp3")
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

    sendVoiceCommand("/tmp/tts.wav".into())?;

    Ok("{}".to_string())
}
