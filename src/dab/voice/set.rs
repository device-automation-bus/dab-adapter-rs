use crate::dab::voice::list::VoiceSystem;
use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct SetVoiceSystemRequest {
    pub voiceSystem: VoiceSystem,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct SetVoiceSystemResponse {
    pub voiceSystem: VoiceSystem,
}
