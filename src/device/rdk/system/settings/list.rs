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
//     pub memc: bool,
//     pub cec: bool,
//     pub lowLatencyMode: bool,
//     pub matchContentFrameRate: Vec<MatchContentFrameRate>,
//     pub hdrOutputMode: Vec<HdrOutputMode>,
//     pub pictureMode: Vec<PictureMode>,
//     pub audioOutputMode: Vec<AudioOutputMode>,
//     pub audioOutputSource: Vec<AudioOutputSource>,
//     pub videoInputSource: Vec<VideoInputSource>,
//     pub audioVolume: AudioVolume,
//     pub mute: bool,
//     pub textToSpeech: bool,
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
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::rdk_request_with_params;
use crate::device::rdk::interface::rdk_sound_mode_to_dab;
use crate::device::rdk::interface::RdkResponse;
use crate::device::rdk::system::settings::get::get_rdk_audio_port;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use super::get::get_rdk_cec;
use super::get::get_rdk_mute;

fn get_rdk_resolutions() -> Result<Vec<OutputResolution>, String> {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetSupportedResolutions {
        supportedResolutions: Vec<String>,
        success: bool,
    }

    let rdkresponse: RdkResponse<GetSupportedResolutions> =
        rdk_request("org.rdk.DisplaySettings.getSupportedResolutions")?;

    lazy_static! {
        static ref RDK_RESOLUTION_MAP: HashMap<&'static str, [u32; 2]> = HashMap::from([
            ("480", [640, 480]),
            ("576", [720, 576]),
            ("720", [1280, 720]),
            ("1080", [1920, 1080]),
            ("2160", [3840, 2160]),
        ]);
    }

    let res = rdkresponse
        .result
        .supportedResolutions
        .iter()
        .filter_map(|item| {
            if let Some((resolution, framerate)) = item.split_once("p") {
                if let Some(dimensions) = RDK_RESOLUTION_MAP.get(resolution) {
                    if let Ok(framerate_n) = framerate.parse::<f32>() {
                        return Some(OutputResolution {
                            width: dimensions[0],
                            height: dimensions[1],
                            frequency: framerate_n,
                        });
                    }
                }
            }
            None
        })
        .collect();

    Ok(res)
}

pub fn get_rdk_supported_audio_source() -> Result<Vec<AudioOutputSource>, String> {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct GetConnectedAudioPorts {
        supportedAudioPorts: Vec<String>,
        success: bool,
    }
    let mut response = vec![AudioOutputSource::default()];
    let rdkresponse: RdkResponse<GetConnectedAudioPorts> =
        rdk_request("org.rdk.DisplaySettings.getSupportedAudioPorts")?;

    for source in rdkresponse.result.supportedAudioPorts.iter() {
        let val = match source.as_str() {
            "SPDIF0" => AudioOutputSource::Optical,
            "HDMI0" => AudioOutputSource::HDMI,
            _ => {
                continue;
            }
        };

        if !response.contains(&val) {
            response.push(val);
        }
    }
    Ok(response)
}

pub fn get_rdk_supported_audio_modes(port: &String) -> Result<Vec<String>, String> {
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        audioPort: String,
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetSupportedAudioModes {
        supportedAudioModes: Vec<String>,
        success: bool,
    }

    let req_params = Param {
        audioPort: port.to_string(),
    };

    let rdkresponse: RdkResponse<GetSupportedAudioModes> =
        rdk_request_with_params("org.rdk.DisplaySettings.getSupportedAudioModes", req_params)?;

    Ok(rdkresponse.result.supportedAudioModes)
}

fn get_rdk_audio_output_modes() -> Result<Vec<AudioOutputMode>, String> {
    let mut res = get_rdk_supported_audio_modes(&get_rdk_audio_port()?)?
        .iter()
        .filter_map(|mode| rdk_sound_mode_to_dab(mode))
        .collect::<Vec<_>>();

    // Ensure the result has at most one AudioOutputMode::MultichannelPcm
    res.sort();
    res.dedup();

    Ok(res)
}

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

    ResponseOperator.outputResolution = get_rdk_resolutions()?;

    ResponseOperator.memc = false;

    ResponseOperator.cec = get_rdk_cec()?;

    ResponseOperator.lowLatencyMode = true;

    ResponseOperator.mute = get_rdk_mute()?;

    ResponseOperator.textToSpeech = true;

    ResponseOperator.hdrOutputMode = vec![
        HdrOutputMode::AlwaysHdr,
        HdrOutputMode::HdrOnPlayback,
        // HdrOutputMode::DisableHdr,
    ];

    ResponseOperator.audioVolume = AudioVolume { min: 0, max: 100 };

    ResponseOperator.matchContentFrameRate = vec![
        MatchContentFrameRate::EnabledAlways,
        // MatchContentFrameRate::EnabledSeamlessOnly,
        // MatchContentFrameRate::Disabled,
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
    ResponseOperator.audioOutputMode = get_rdk_audio_output_modes()?;
    ResponseOperator.audioOutputSource = get_rdk_supported_audio_source()?;

    // vec![
    //     AudioOutputSource::NativeSpeaker,
    //     AudioOutputSource::Arc,
    //     AudioOutputSource::EArc,
    //     AudioOutputSource::Optical,
    //     AudioOutputSource::Aux,
    //     AudioOutputSource::Bluetooth,
    //     AudioOutputSource::Auto,
    //     AudioOutputSource::HDMI,
    // ];
    ResponseOperator.videoInputSource = vec![
        //VideoInputSource::Tuner,
        // VideoInputSource::HDMI1,
        // VideoInputSource::HDMI2,
        // VideoInputSource::HDMI3,
        // VideoInputSource::HDMI4,
        // VideoInputSource::Composite,
        // VideoInputSource::Component,
        VideoInputSource::Home,
        // VideoInputSource::Cast,
    ];

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
