use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct SendAudioRequest {
    pub fileLocation: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct VoiceRequestResponse {}
