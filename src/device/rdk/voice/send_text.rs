use super::voice_functions::sendVoiceCommand;
use crate::dab::structs::DabError;
use crate::dab::structs::SendTextRequest;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use surf::Url;
use tokio::runtime::Runtime;
use urlencoding::encode;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: SendTextRequest) -> Result<String, DabError> {
    // TODO: Add other RDK specific voice protocol support confirmation.
    if _dab_request.voiceSystem.to_string() != "AmazonAlexa" {
        return Err(DabError::Err400("Unsupported 'voiceSystem'.".to_string()));
    }

    // Use Google Translate TTS to generate the voice file.
    let len = _dab_request.requestText.len();
    let encoded_text = encode(&_dab_request.requestText).into_owned();
    let mp3_file_path = Path::new("/tmp/tts.mp3");
    let url = format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=en&total=1&idx=0&textlen={}&tl=en&client=tw-ob",&encoded_text,len);

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let parsed_url = Url::parse(&url).map_err(|e| e.to_string())?;
        let mut res = surf::get(parsed_url).await.map_err(|e| e.to_string())?;
        let body = res.body_bytes().await.map_err(|e| e.to_string())?;
        let mut file = File::create(mp3_file_path).map_err(|e| e.to_string())?;
        file.write_all(&body).map_err(|e| e.to_string())?;
        Ok(())
    })
    .map_err(|e| DabError::Err500(e))?;

    // Convert the mp3 file to wav.

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

    let voiceresponse = sendVoiceCommand("/tmp/tts.wav".into());
    match voiceresponse {
        Ok(_) => (),
        Err(e) => { return Err(e); },
    }

    Ok("{}".to_string())
}
