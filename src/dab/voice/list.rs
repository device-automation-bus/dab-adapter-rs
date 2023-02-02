use serde::Serialize;
#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct VoiceListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize)]
pub struct VoiceSystem {
    pub name: String,
    pub enabled: bool,
}
