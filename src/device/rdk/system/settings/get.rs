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
#[allow(unused_imports)]
use crate::dab::structs::ErrorResponse;
use crate::dab::structs::OutputResolution;
#[allow(unused_imports)]
use crate::dab::structs::GetSystemSettingsRequest;
use crate::dab::structs::GetSystemSettingsResponse;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::rdk_request_with_params;
use crate::device::rdk::interface::rdk_sound_mode_to_dab;
use crate::device::rdk::interface::RdkResponse;
use crate::device::rdk::interface::service_activate;
use crate::device::rdk::interface::service_deactivate;
use serde::{Serialize, Deserialize};
use serde_json::json;

fn get_rdk_language() -> Result<String, String> {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct GetUILanguage {
        ui_language: String,
        success: bool,
    }

    let rdkresponse: RdkResponse<GetUILanguage> = rdk_request("org.rdk.UserPreferences.1.getUILanguage")?;

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

    let rdkresponse: RdkResponse<GetDisplayFrameRate> = rdk_request("org.rdk.FrameRate.getDisplayFrameRate")?;

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

    rdkresponse.result.connectedAudioPorts
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

pub fn get_rdk_mute() -> Result<bool, String> {
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

pub fn get_rdk_cec() -> Result<bool, String> {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct CecGetEnabled {
        enabled: bool,
        success: bool,
    }

    let rdkresponse: RdkResponse<CecGetEnabled> = rdk_request("org.rdk.HdmiCec_2.getEnabled")?;

    Ok(rdkresponse.result.enabled)
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
        None => Err(format!("Unknown RDK sound mode {}", rdkresponse.result.soundMode)),
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
    response.audioOutputMode = get_rdk_audio_output_mode()?;

    let mut response_json = json!(response);
    response_json["status"] = json!(200);
    Ok(serde_json::to_string(&response_json).unwrap())
}
