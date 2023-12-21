// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct GetSystemSettingsRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize,Deserialize)]
// pub struct GetSystemSettingsResponse{
// pub language: String,
// pub outputResolution: OutputResolution,
// pub memc: bool,
// pub cec: bool,
// pub lowLatencyMode: bool,
// pub matchContentFrameRate: MatchContentFrameRate,
// pub hdrOutputMode: HdrOutputMode,
// pub pictureMode: PictureMode,
// pub audioOutputMode: AudioOutputMode,
// pub audioOutputSource: AudioOutputSource,
// pub videoInputSource: VideoInputSource,
// pub audioVolume: u32,
// pub mute: bool,
// pub textToSpeech: bool,
// }

use crate::dab::structs::AudioOutputMode;
use crate::dab::structs::AudioOutputSource;
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
#[allow(unused_imports)]
use crate::dab::structs::GetSystemSettingsRequest;
use crate::dab::structs::GetSystemSettingsResponse;
use crate::dab::structs::HdrOutputMode;
use crate::dab::structs::OutputResolution;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::rdk_request_with_params;
use crate::device::rdk::interface::rdk_sound_mode_to_dab;
use crate::device::rdk::interface::service_activate;
use crate::device::rdk::interface::service_deactivate;
use crate::device::rdk::interface::RdkResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

fn get_rdk_language() -> Result<String, String> {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetUILanguage {
        ui_language: String,
        success: bool,
    }

    let rdkresponse: RdkResponse<GetUILanguage> =
        rdk_request("org.rdk.UserPreferences.1.getUILanguage")?;

    Ok(rdkresponse.result.ui_language)
}

fn get_rdk_resolution() -> Result<OutputResolution, String> {
    service_activate("org.rdk.FrameRate".to_string()).unwrap();

    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetDisplayFrameRate {
        framerate: String,
        success: bool,
    }

    let rdkresponse: RdkResponse<GetDisplayFrameRate> =
        rdk_request("org.rdk.FrameRate.getDisplayFrameRate")?;

    let mut dimensions = rdkresponse
        .result
        .framerate
        .trim_end_matches(']')
        .split('x');

    service_deactivate("org.rdk.RDKShell.getDisplayFrameRate".to_string()).unwrap();

    Ok(OutputResolution {
        width: dimensions.next().unwrap().parse::<i32>().unwrap() as u32,
        height: dimensions.next().unwrap().parse::<i32>().unwrap() as u32,
        frequency: dimensions.next().unwrap().parse::<i32>().unwrap() as f32,
    })
}

pub fn get_rdk_connected_video_displays() -> Result<String, String> {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct ConnectedVideoDisplays {
        connectedVideoDisplays: Vec<String>,
        success: bool,
    }

    let rdkresponse: RdkResponse<ConnectedVideoDisplays> =
        rdk_request("org.rdk.DisplaySettings.getConnectedVideoDisplays")?;

    rdkresponse
        .result
        .connectedVideoDisplays
        .get(0)
        .cloned()
        .ok_or("Device doesn't have any connected video port.".into())
}

pub fn get_rdk_hdr_current_setting() -> Result<HdrOutputMode, String> {
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

    if settop_hdr_response.result.supportsHDR & tv_hdr_response.result.supportsHDR {
        Ok(HdrOutputMode::AlwaysHdr)
    } else {
        Ok(HdrOutputMode::DisableHdr)
    }
}

pub fn get_rdk_audio_port() -> Result<String, String> {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetConnectedAudioPorts {
        connectedAudioPorts: Vec<String>,
        success: bool,
    }

    let rdkresponse: RdkResponse<GetConnectedAudioPorts> =
        rdk_request("org.rdk.DisplaySettings.getConnectedAudioPorts")?;

    rdkresponse
        .result
        .connectedAudioPorts
        .get(0)
        .cloned()
        .ok_or("Device doesn't have any connected audio port.".into())
}

fn get_rdk_audio_volume() -> Result<u32, String> {
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        audioPort: String,
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetVolumeLevel {
        volumeLevel: String,
        success: bool,
    }

    let req_params = Param {
        audioPort: get_rdk_audio_port()?,
    };

    let rdkresponse: RdkResponse<GetVolumeLevel> =
        rdk_request_with_params("org.rdk.DisplaySettings.getVolumeLevel", req_params)?;

    match rdkresponse.result.volumeLevel.parse::<f32>() {
        Ok(volume) => Ok(volume as u32),
        Err(error) => Err(error.to_string()),
    }
}

fn get_rdk_mute() -> Result<bool, String> {
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        audioPort: String,
    }

    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetMuted {
        muted: bool,
        success: bool,
    }

    let req_params = Param {
        audioPort: get_rdk_audio_port()?,
    };

    let rdkresponse: RdkResponse<GetMuted> =
        rdk_request_with_params("org.rdk.DisplaySettings.getMuted", req_params)?;

    Ok(rdkresponse.result.muted)
}

pub fn get_rdk_tts() -> Result<bool, String> {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct TtsGetEnabled {
        isenabled: bool,
        TTS_Status: u16,
        success: bool,
    }

    let rdkresponse: RdkResponse<TtsGetEnabled> = rdk_request("org.rdk.TextToSpeech.isttsenabled")?;

    Ok(rdkresponse.result.isenabled)
}

fn get_rdk_cec() -> Result<bool, String> {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct CecGetEnabled {
        enabled: bool,
        success: bool,
    }

    let rdkresponse: RdkResponse<CecGetEnabled> = rdk_request("org.rdk.HdmiCec_2.getEnabled")?;

    Ok(rdkresponse.result.enabled)
}

fn get_rdk_connected_audio_source() -> Result<AudioOutputSource, String> {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct GetConnectedAudioPorts {
        connectedAudioPorts: Vec<String>,
        success: bool,
    }
    let mut response = vec![AudioOutputSource::default()];

    let rdkresponse: RdkResponse<GetConnectedAudioPorts> =
        rdk_request("org.rdk.DisplaySettings.getConnectedAudioPorts")?;

    for source in rdkresponse.result.connectedAudioPorts.iter() {
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
    Ok(response.get(0).unwrap().clone())
}

fn get_rdk_audio_output_mode() -> Result<AudioOutputMode, String> {
    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct Param {
        audioPort: String,
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetSoundMode {
        soundMode: String,
        success: bool,
    }

    let req_params = Param {
        audioPort: get_rdk_audio_port()?,
    };

    let rdkresponse: RdkResponse<GetSoundMode> =
        rdk_request_with_params("org.rdk.DisplaySettings.getSoundMode", req_params)?;

    match rdk_sound_mode_to_dab(&rdkresponse.result.soundMode) {
        Some(mode) => Ok(mode),
        None => Err(format!(
            "Unknown RDK sound mode {}",
            rdkresponse.result.soundMode
        )),
    }
}

pub fn process(_packet: String) -> Result<String, String> {
    let mut response = GetSystemSettingsResponse::default();
    // *** Fill in the fields of the struct GetSystemSettingsResponse here ***

    response.language = get_rdk_language()?;
    response.outputResolution = get_rdk_resolution()?;
    response.audioVolume = get_rdk_audio_volume()?;
    response.mute = get_rdk_mute()?;
    response.cec = get_rdk_cec()?;
    response.hdrOutputMode = get_rdk_hdr_current_setting()?;
    response.audioOutputMode = get_rdk_audio_output_mode()?;
    response.audioOutputSource = get_rdk_connected_audio_source()?;
    response.lowLatencyMode = true;
    response.textToSpeech = get_rdk_tts()?;

    let mut response_json = json!(response);
    response_json["status"] = json!(200);
    Ok(serde_json::to_string(&response_json).unwrap())
}
