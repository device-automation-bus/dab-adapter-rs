// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct SettingsListRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct OutputResolution{
// pub width: u32,
// pub height: u32,
// pub frequency: f32,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum MatchContentFrameRate{#[default]
//     EnabledAlways,
//     EnabledSeamlessOnly,
//     Disabled,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum HdrOutputMode{#[default]
//     AlwaysHdr,
//     HdrOnPlayback,
//     DisableHdr,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum PictureMode{#[default]
//     Standard,
//     Dynamic,
//     Movie,
//     Sports,
//     FilmMaker,
//     Game,
//     Auto,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum AudioOutputMode{#[default]
//     Stereo,
//     MultichannelPcm,
//     PassThrough,
//     Auto,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum AudioOutputSource{#[default]
//     NativeSpeaker,
//     Arc,
//     EArc,
//     Optical,
//     Aux,
//     Bluetooth,
//     Auto,
//     HDMI,
// }

// #[allow(dead_code)]
// #[derive(Default,Serialize,Deserialize)]
// pub enum VideoInputSource{#[default]
//     Tuner,
//     HDMI1,
//     HDMI2,
//     HDMI3,
//     HDMI4,
//     Composite,
//     Component,
//     Home,
//     Cast,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct AudioVolume{
// pub min: u32,
// pub max: u32,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct ListSystemSettings {
//     pub language: Vec<String>,
//     pub outputResolution: Vec<OutputResolution>,
//     pub memc: Vec<bool>,
//     pub cec: Vec<bool>,
//     pub lowLatencyMode: Vec<bool>,
//     pub matchContentFrameRate: Vec<MatchContentFrameRate>,
//     pub hdrOutputMode: Vec<HdrOutputMode>,
//     pub pictureMode: Vec<PictureMode>,
//     pub audioOutputMode: Vec<AudioOutputMode>,
//     pub audioOutputSource: Vec<AudioOutputSource>,
//     pub videoInputSource: Vec<VideoInputSource>,
//     pub audioVolume: AudioVolume,
//     pub mute: Vec<bool>,
//     pub textToSpeech: Vec<bool>,
// }

// use super::LANGUAGES;
// use super::RESOLUTIONS;
use crate::dab::structs::AudioOutputMode;
use crate::dab::structs::AudioOutputSource;
use crate::dab::structs::AudioVolume;
use crate::dab::structs::HdrOutputMode;
use crate::dab::structs::ListSystemSettings;
use crate::dab::structs::MatchContentFrameRate;
use crate::dab::structs::OutputResolution;
use crate::dab::structs::PictureMode;
use crate::dab::structs::VideoInputSource;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = ListSystemSettings::default();
    // *** Fill in the fields of the struct ListSystemSettings here ***

    // // Return language tags defined in RFC 5646.
    // /*
    //     IMPORTANT NOTE: As defined on the org.rdk.UserPreferences plugin documentation
    //     (https://rdkcentral.github.io/rdkservices/#/api/UserPreferencesPlugin):
    //     "The language is written to the /opt/user_preferences.conf file on the device.
    //     It is the responsibility of the client application to validate the language value and process
    //     it if required. Any language string that is valid on the client can be set"
    ResponseOperator.language = vec!["en-US".to_string()];

    ResponseOperator.outputResolution = vec![
        OutputResolution {
            width: 1920,
            height: 1080,
            frequency: 60.0,
        },
        OutputResolution {
            width: 1920,
            height: 1080,
            frequency: 50.0,
        },
        OutputResolution {
            width: 1280,
            height: 720,
            frequency: 60.0,
        },
        OutputResolution {
            width: 1280,
            height: 720,
            frequency: 50.0,
        },
        OutputResolution {
            width: 720,
            height: 576,
            frequency: 50.0,
        },
        OutputResolution {
            width: 640,
            height: 480,
            frequency: 50.0,
        },
    ];

    ResponseOperator.memc = vec![false, true];

    ResponseOperator.cec = vec![false, true];

    ResponseOperator.lowLatencyMode = vec![false, true];

    ResponseOperator.mute = vec![false, true];

    ResponseOperator.textToSpeech = vec![false, true];

    ResponseOperator.hdrOutputMode = vec![
        HdrOutputMode::AlwaysHdr,
        HdrOutputMode::HdrOnPlayback,
        HdrOutputMode::DisableHdr,
    ];

    ResponseOperator.audioVolume = AudioVolume { min: 0, max: 100 };

    ResponseOperator.matchContentFrameRate = vec![
        MatchContentFrameRate::EnabledAlways,
        // MatchContentFrameRate::EnabledSeamlessOnly,
        MatchContentFrameRate::Disabled,
    ];

    ResponseOperator.pictureMode = vec![
        PictureMode::Standard,
        // PictureMode::Dynamic,
        // PictureMode::Movie,
        // PictureMode::Sports,
        // PictureMode::FilmMaker,
        // PictureMode::Game,
        // PictureMode::Auto,
    ];
    ResponseOperator.audioOutputMode = vec![
        AudioOutputMode::Stereo,
        AudioOutputMode::MultichannelPcm,
        AudioOutputMode::PassThrough,
        AudioOutputMode::Auto,
    ];
    ResponseOperator.audioOutputSource = vec![
        // AudioOutputSource::NativeSpeaker,
        // AudioOutputSource::Arc,
        // AudioOutputSource::EArc,
        AudioOutputSource::Optical,
        // AudioOutputSource::Aux,
        AudioOutputSource::Bluetooth,
        // AudioOutputSource::Auto,
        AudioOutputSource::HDMI,
    ];
    ResponseOperator.videoInputSource = vec![
        VideoInputSource::Tuner,
        // VideoInputSource::HDMI1,
        // VideoInputSource::HDMI2,
        // VideoInputSource::HDMI3,
        // VideoInputSource::HDMI4,
        // VideoInputSource::Composite,
        // VideoInputSource::Component,
        // VideoInputSource::Home,
        // VideoInputSource::Cast,
    ];

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
