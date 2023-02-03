use crate::dab::system::settings::list::AudioOutputMode;
use crate::dab::system::settings::list::AudioOutputSource;
use crate::dab::system::settings::list::HdrOutputMode;
use crate::dab::system::settings::list::MatchContentFrameRate;
use crate::dab::system::settings::list::OutputResolution;
use crate::dab::system::settings::list::PictureMode;
use crate::dab::system::settings::list::VideoInputSource;
use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetSystemSettingsRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetSystemSettingsResponse {
    pub language: String,
    pub outputResolution: OutputResolution,
    pub memc: bool,
    pub cec: bool,
    pub lowLatencyMode: bool,
    pub matchContentFrameRate: MatchContentFrameRate,
    pub hdrOutputMode: HdrOutputMode,
    pub pictureMode: PictureMode,
    pub audioOutputMode: AudioOutputMode,
    pub audioOutputSource: AudioOutputSource,
    pub videoInputSource: VideoInputSource,
    pub audioVolume: u32,
    pub mute: bool,
    pub textToSpeech: bool,
}
