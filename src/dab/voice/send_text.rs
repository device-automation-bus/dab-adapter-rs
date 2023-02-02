use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct SendTextRequest {
    pub requestText: String,
    pub voiceSystem: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct VoiceTextRequestResponse {}
