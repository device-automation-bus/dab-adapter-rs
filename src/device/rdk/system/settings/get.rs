use crate::dab::structs::AudioOutputMode;
use crate::dab::structs::AudioOutputSource;
use crate::dab::structs::DabError;
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

fn get_rdk_language() -> Result<String, DabError> {
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

fn get_rdk_resolution() -> Result<OutputResolution, DabError> {
    service_activate("org.rdk.FrameRate".to_string())?;

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

    service_deactivate("org.rdk.RDKShell.getDisplayFrameRate".to_string())?;

    Ok(OutputResolution {
        width: dimensions.next().unwrap().parse::<i32>().unwrap() as u32,
        height: dimensions.next().unwrap().parse::<i32>().unwrap() as u32,
        frequency: dimensions.next().unwrap().parse::<i32>().unwrap() as f32,
    })
}

pub fn get_rdk_connected_video_displays() -> Result<String, DabError> {
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
        .ok_or(DabError::Err500(
            "Device doesn't have any connected video port.".to_string(),
        ))
}

pub fn get_rdk_hdr_current_setting() -> Result<HdrOutputMode, DabError> {
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

pub fn get_rdk_audio_port() -> Result<String, DabError> {
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
        .ok_or(DabError::Err500(
            "Device doesn't have any connected audio port.".to_string(),
        ))
}

fn get_rdk_audio_volume() -> Result<u32, DabError> {
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
        Err(error) => Err(DabError::Err500(error.to_string())),
    }
}

fn get_rdk_mute() -> Result<bool, DabError> {
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

pub fn get_rdk_tts() -> Result<bool, DabError> {
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

pub fn get_rdk_cec() -> Result<bool, DabError> {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct CecGetEnabled {
        enabled: bool,
        success: bool,
    }

    let rdkresponse: RdkResponse<CecGetEnabled> = rdk_request("org.rdk.HdmiCec_2.getEnabled")?;

    Ok(rdkresponse.result.enabled)
}

fn get_rdk_connected_audio_source() -> Result<AudioOutputSource, DabError> {
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

fn get_rdk_audio_output_mode() -> Result<AudioOutputMode, DabError> {
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
        None => Err(DabError::Err500(format!(
            "Unknown RDK sound mode {}",
            rdkresponse.result.soundMode
        ))),
    }
}

pub fn process(_dab_request: GetSystemSettingsRequest) -> Result<String, DabError> {
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
    response.lowLatencyMode = false;
    response.textToSpeech = get_rdk_tts()?;

    Ok(serde_json::to_string(&response).unwrap())
}
