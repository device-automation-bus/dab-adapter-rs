use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct SettingsListRequest {}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct OutputResolution{
	pub width: u32,
	pub height: u32,
	pub frequency: f32,
}

#[allow(dead_code)]
#[derive(Default,Serialize,Deserialize)]
pub enum MatchContentFrameRate{#[default]
	    EnabledAlways,
	    EnabledSeamlessOnly,
	    Disabled,
}

#[allow(dead_code)]
#[derive(Default,Serialize,Deserialize)]
pub enum HdrOutputMode{#[default]
	    AlwaysHdr,
	    HdrOnPlayback,
	    DisableHdr,
}

#[allow(dead_code)]
#[derive(Default,Serialize,Deserialize)]
pub enum PictureMode{#[default]
	    Standard,
	    Dynamic,
	    Movie,
	    Sports,
	    FilmMaker,
	    Game,
	    Auto,
}

#[allow(dead_code)]
#[derive(Default,Serialize,Deserialize)]
pub enum AudioOutputMode{#[default]
	    Stereo,
	    MultichannelPcm,
	    PassThrough,
	    Auto,
}

#[allow(dead_code)]
#[derive(Default,Serialize,Deserialize)]
pub enum AudioOutputSource{#[default]
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
#[derive(Default,Serialize,Deserialize)]
pub enum VideoInputSource{#[default]
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
#[derive(Default,Serialize,Deserialize)]
pub struct AudioVolume{
	pub min: u32,
	pub max: u32,
}

#[allow(non_snake_case)]
#[derive(Default,Serialize,Deserialize)]
pub struct ListSystemSettings{
	pub language: Vec<String>,
	pub outputResolution: Vec<OutputResolution>,
	pub memc: bool,
	pub cec: bool,
	pub lowLatencyMode: bool,
	pub matchContentFrameRate: Vec<MatchContentFrameRate>,
	pub hdrOutputMode: Vec<HdrOutputMode>,
	pub pictureMode: Vec<PictureMode>,
	pub audioOutputMode: Vec<AudioOutputMode>,
	pub audioOutputSource: Vec<AudioOutputSource>,
	pub videoInputSource: Vec<VideoInputSource>,
	pub audioVolume: AudioVolume,
	pub mute: bool,
	pub textToSpeech: bool,
}

