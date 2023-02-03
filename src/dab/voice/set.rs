use crate::dab::voice::list::VoiceSystem;
use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SetVoiceSystemRequest {
    pub voiceSystem: VoiceSystem,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SetVoiceSystemResponse {
    pub voiceSystem: VoiceSystem,
}
