use crate::dab::structs::AudioOutputMode;
use crate::dab::structs::AudioOutputSource;
use crate::dab::structs::AudioVolume;
use crate::dab::structs::DabError;
use crate::dab::structs::HdrOutputMode;
use crate::dab::structs::ListSystemSettingsRequest;
use crate::dab::structs::ListSystemSettingsResponse;
use crate::dab::structs::MatchContentFrameRate;
use crate::dab::structs::OutputResolution;
use crate::dab::structs::VideoInputSource;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::rdk_request_with_params;
use crate::device::rdk::interface::rdk_sound_mode_to_dab;
use crate::device::rdk::interface::service_is_available;
use crate::device::rdk::interface::RdkResponse;
use crate::device::rdk::system::settings::get::get_rdk_audio_port;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::get::get_rdk_tts;

fn get_rdk_resolutions() -> Result<Vec<OutputResolution>, DabError> {
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

pub fn get_rdk_hdr_settings() -> Result<Vec<HdrOutputMode>, DabError> {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct GetHDRSupport {
        standards: Vec<String>,
        supportsHDR: bool,
        success: bool,
    }

    let settop_hdr_response: RdkResponse<GetHDRSupport> =
        rdk_request("org.rdk.DisplaySettings.getSettopHDRSupport")?;
    let tv_hdr_response: RdkResponse<GetHDRSupport> =
        rdk_request("org.rdk.DisplaySettings.getTvHDRSupport")?;

    let mut response = vec![HdrOutputMode::DisableHdr];

    if settop_hdr_response.result.supportsHDR & tv_hdr_response.result.supportsHDR {
        response.insert(0, HdrOutputMode::AlwaysHdr);
    }

    Ok(response)
}

pub fn get_rdk_supported_audio_source() -> Result<Vec<AudioOutputSource>, DabError> {
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
            "IDLR0" => AudioOutputSource::Aux,
            "SPEAKER0" => AudioOutputSource::NativeSpeaker,
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

pub fn get_rdk_supported_audio_modes(port: &String) -> Result<Vec<String>, DabError> {
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

fn get_rdk_audio_output_modes() -> Result<Vec<AudioOutputMode>, DabError> {
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
pub fn process(_dab_request: ListSystemSettingsRequest) -> Result<String, DabError> {
    let mut ResponseOperator = ListSystemSettingsResponse::default();
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

    ResponseOperator.cec = service_is_available("org.rdk.HdmiCecSource")?;

    ResponseOperator.lowLatencyMode = false;

    ResponseOperator.mute = true;

    ResponseOperator.textToSpeech = get_rdk_tts()?;

    ResponseOperator.hdrOutputMode = get_rdk_hdr_settings()?;

    ResponseOperator.audioVolume = AudioVolume { min: 0, max: 100 };

    ResponseOperator.matchContentFrameRate = vec![
        MatchContentFrameRate::EnabledAlways,
        // MatchContentFrameRate::EnabledSeamlessOnly,
        // MatchContentFrameRate::Disabled,
    ];

    ResponseOperator.pictureMode = vec![
        // PictureMode::Standard,
        // PictureMode::Dynamic,
        // PictureMode::Movie,
        // PictureMode::Sports,
        // PictureMode::FilmMaker,
        // PictureMode::Game,
        // PictureMode::Auto,
    ];
    ResponseOperator.audioOutputMode = get_rdk_audio_output_modes()?;
    ResponseOperator.audioOutputSource = get_rdk_supported_audio_source()?;
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
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
