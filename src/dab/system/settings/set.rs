use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SettingsGetRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub struct OutputResolution {
    pub width: u32,
    pub height: u32,
    pub frequency: f32,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub enum MatchContentFrameRate {
    #[default]
    EnabledAlways,
    EnabledSeamlessOnly,
    Disabled,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub enum HdrOutputMode {
    #[default]
    AlwaysHdr,
    HdrOnPlayback,
    DisableHdr,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub enum PictureMode {
    #[default]
    Standard,
    Dynamic,
    Movie,
    Sports,
    FilmMaker,
    Game,
    Auto,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub enum AudioOutputMode {
    #[default]
    Stereo,
    MultichannelPcm,
    PassThrough,
    Auto,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub enum AudioOutputSource {
    #[default]
    NativeSpeaker,
    Arc,
    EArc,
    Optical,
    Aux,
    Bluetooth,
    Auto,
    HDMI,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub enum VideoInputSource {
    #[default]
    Tuner,
    HDMI1,
    HDMI2,
    HDMI3,
    HDMI4,
    Composite,
    Component,
    Home,
    Cast,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize, PartialEq)]
pub struct AudioVolume {
    pub min: u32,
    pub max: u32,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SetSystemSettingsRequest {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub outputResolution: OutputResolution,
    #[serde(default)]
    pub memc: bool,
    #[serde(default)]
    pub cec: bool,
    #[serde(default)]
    pub lowLatencyMode: bool,
    #[serde(default)]
    pub matchContentFrameRate: MatchContentFrameRate,
    #[serde(default)]
    pub hdrOutputMode: HdrOutputMode,
    #[serde(default)]
    pub pictureMode: PictureMode,
    #[serde(default)]
    pub audioOutputMode: AudioOutputMode,
    #[serde(default)]
    pub audioOutputSource: AudioOutputSource,
    #[serde(default)]
    pub videoInputSource: VideoInputSource,
    #[serde(default)]
    pub audioVolume: u32,
    #[serde(default)]
    pub mute: bool,
    #[serde(default)]
    pub textToSpeech: bool,
}
